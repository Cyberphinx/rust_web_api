use rust_web_api::{app_state::AppState, run};

#[tokio::main]
async fn main() {
    let app_state = AppState::build().await;
    match app_state {
        Ok(app_state) => run(app_state).await,
        Err(error) => {
            eprintln!("Error instantiating app states: {}", error.message);
            panic!()
        }
    }
}
