

# aide-axum-typed-multipart-2

Envoltorio en torno a [`axum_typed_multipart`](https://docs.rs/axum_typed_multipart/0.11.0/axum_typed_multipart/) para generar documentación para solicitudes multipart.

## Ejemplo

```rust
use aide_axum_typed_multipart::{FieldData, TypedMultipart};
use axum::{http::StatusCode};
use bytes::Bytes;
use axum_typed_multipart::TryFromMultipart;
use schemars::JsonSchema;

#[derive(TryFromMultipart, JsonSchema)]
struct MyMultipart {
    title: String,
    description: String,
    #[form_data(limit = "unlimited")]
    image: FieldData<Bytes>,
}

async fn post_hello_world(_: TypedMultipart<MyMultipart>) -> StatusCode {
    // do something
    return StatusCode::OK;
}
```

## Licencia

Todo el código en este repositorio está licenciado de forma dual bajo [MIT](./LICENSE-MIT) y [Apache-2.0](./LICENSE-Apache).
