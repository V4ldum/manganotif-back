use axum::response::{IntoResponse, Response};
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
};

use crate::utils::APIError;

pub(crate) async fn check_auth(
    State(api_secret): State<String>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    if cfg!(not(debug_assertions)) {
        // No check in debug mode
        let Some(key) = headers.get("X-API-Key") else {
            return APIError::no_api_key().into_response();
        };
        let Ok(key) = key.to_str() else {
            return APIError::bad_api_key().into_response();
        };

        // Wrong API key
        if key != api_secret {
            return APIError::bad_api_key().into_response();
        }
    }

    next.run(request).await
}
