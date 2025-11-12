mod global_manga;
mod health_check;
mod manga;
mod manga_not_found;

use crate::{
    AppState,
    middleware::check_auth,
    routes::{global_manga::get_all_global_mangas, manga::get_all_mangas},
};
use axum::{Router, middleware::from_fn_with_state, routing::get};
use health_check::health_check;
use manga_not_found::get_all_mangas_not_found;

pub(super) fn router(state: AppState) -> axum::Router {
    //let cors = CorsLayer::new()
    //    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //    .allow_origin(Any);

    Router::new()
        .route("/adm/manga", get(get_all_mangas))
        .route("/adm/manga/not_found", get(get_all_mangas_not_found))
        .route("/adm/manga/global", get(get_all_global_mangas))
        // Anything above needs authentication
        .route_layer(from_fn_with_state(state.clone(), check_auth))
        // Anything above can use the state
        .with_state(state)
        .route("/health", get(health_check))
    //.layer(cors)
}
