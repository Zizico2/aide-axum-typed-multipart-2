use std::{borrow::Cow, ops::Deref};

use aide::{
    OperationInput,
    generate::GenContext,
    openapi::{MediaType, Operation, RequestBody, SchemaObject},
    operation::set_body,
};
use axum::extract::{FromRequest, Request, multipart::Field};
use axum_typed_multipart::{TryFromField, TryFromMultipart, async_trait};
use indexmap::IndexMap;
use schemars::JsonSchema;

/// Drop-in replacement for [`axum_typed_multipart::TypedMultipart`](https://docs.rs/axum_typed_multipart/0.11.0/axum_typed_multipart/struct.TypedMultipart.html)
/// that implements [`OperationInput`](https://docs.rs/aide/latest/aide/operation/trait.OperationInput.html)
#[derive(Debug)]
pub struct TypedMultipart<T>(pub axum_typed_multipart::TypedMultipart<T>);

impl<T> Deref for TypedMultipart<T> {
    type Target = axum_typed_multipart::TypedMultipart<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequest<S> for TypedMultipart<T>
where
    T: TryFromMultipart,
    S: Send + Sync,
{
    type Rejection = axum_typed_multipart::TypedMultipartError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let a = axum_typed_multipart::TypedMultipart::from_request(req, state).await?;
        Ok(Self(a))
    }
}

impl<T> OperationInput for TypedMultipart<T>
where
    T: JsonSchema,
{
    fn operation_input(ctx: &mut GenContext, operation: &mut Operation) {
        let schema = ctx.schema.subschema_for::<T>();
        let resolved_schema = ctx.resolve_schema(&schema);

        set_body(
            ctx,
            operation,
            RequestBody {
                description: resolved_schema
                    .get("description")
                    .and_then(|d| d.as_str())
                    .map(String::from),
                content: IndexMap::from_iter([(
                    "multipart/form-data".into(),
                    MediaType {
                        schema: Some(SchemaObject {
                            json_schema: schema,
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]),
                required: true,
                extensions: IndexMap::default(),
            },
        );
    }
}

/// Drop-in replacement for [`axum_typed_multipart::FieldData`](https://docs.rs/axum_typed_multipart/0.11.0/axum_typed_multipart/struct.FieldData.html)
/// that implements [`JsonSchema`](https://docs.rs/schemars/0.8.16/schemars/trait.JsonSchema.html)
#[derive(Debug)]
pub struct FieldData<T>(pub axum_typed_multipart::FieldData<T>);

impl<T> Deref for FieldData<T> {
    type Target = axum_typed_multipart::FieldData<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T: TryFromField> TryFromField for FieldData<T> {
    async fn try_from_field(
        field: Field<'_>,
        limit_bytes: Option<usize>,
    ) -> Result<Self, axum_typed_multipart::TypedMultipartError> {
        Ok(Self(
            axum_typed_multipart::FieldData::try_from_field(field, limit_bytes).await?,
        ))
    }
}

impl<T: JsonSchema> JsonSchema for FieldData<T> {
    fn schema_name() -> Cow<'static, str> {
        T::schema_name()
    }

    fn json_schema(r#gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        T::json_schema(r#gen)
    }
}
