use app_state::AppState;
use router::create_router;
use std::{env, net::SocketAddr};

pub mod app_state;
pub mod middleware;
pub mod models;
pub mod queries;
pub mod router;
pub mod routes;
pub mod utilities;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);
    let address: SocketAddr = if env::var("ENVIRONMENT").unwrap().eq("production") {
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
    } else {
        SocketAddr::from(([0, 0, 0, 0], 4000))
    };
    println!("Running on address: {}", &address);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
