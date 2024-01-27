use axum::{routing::get, Router};

use crate::{app_state::AppState, routes::hello::HelloRouter};

/// build the router
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(HelloRouter::hello_world))
        .with_state(app_state)
}
