mod database;
mod middleware;
mod routes;
mod state;
mod utils;

use crate::routes::router;
pub use database::Database;
pub use state::AppState;

pub async fn run(state: AppState) {
    let router = router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("The listener should be able to bind to this port");

    println!("Serving 0.0.0.0:7878");
    axum::serve(listener, router)
        .await
        .expect("The server should launch successfully")
}
