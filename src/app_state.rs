use std::env;

use axum::extract::FromRef;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::utilities::{app_error::AppError, token_wrapper::TokenWrapper};

/// Project AppState struct to provide cross-cutting values across the app
#[derive(Clone, FromRef)]
pub struct AppState {
    pub http_client: reqwest::Client,
    pub pool: Pool<Postgres>,
    pub jwt_secret: TokenWrapper,
}

/// Using Rust Builder Pattern to construct a new instance of [`AppState`].
impl AppState {
    pub async fn build() -> Result<AppState, AppError> {
        // get environment variables from .env file
        // check the existence of .env file
        let database_url = match dotenvy::dotenv() {
            Ok(_) => env::var("DATABASE_URL").expect("to have database url"),
            Err(_) => {
                // fly io
                // if .env file doesnt exist, it means that it is running on fly.io
                env::var("DATABASE_URL")
                    .unwrap()
                    .replace("flycast", "internal")
            }
        };

        // get current development / production environment
        let app_env = env::var("ENVIRONMENT").expect("to have environment");
        println!("Running in {} mode!", &app_env);

        // connect to database via sqlx
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Error connecting to the database");

        // get jwt secret from environment variables
        let jwt_secret = env::var("JWT_SECRET").expect("to have jwt secret");

        // instantiate a http client with reqwest crate
        let http_client = match reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .build() {
            Ok(http_client) => http_client,
            Err(error) => {
                eprintln!("Error building reqwest http client: {:?}", error);
                panic!();
        }
    };

        // returns the new AppState instance
        Ok(AppState {
            http_client,
            pool,
            jwt_secret: TokenWrapper(jwt_secret),
        })
    }
}
