use axum::response::{IntoResponse, Response};

pub(super) async fn health_check() -> Response {
    "OK".into_response()
}
