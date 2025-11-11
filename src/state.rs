use axum::extract::FromRef;

use crate::Database;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: Database,
    pub api_secret: String,
}
