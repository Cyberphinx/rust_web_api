use std::env;

use rust_web_api::{models::user::RequestLoginUser, utilities::app_error::AppError};

pub struct TestSetup {
    pub http_client: reqwest::Client,
    pub logins: RequestLoginUser,
}

impl TestSetup {
    pub fn setup() -> Result<TestSetup, AppError> {
        // load .env file
        let _ = dotenvy::dotenv();

        // credentials
        let email = env::var("RUST_WEB_API_EMAIL").expect("to have login email");
        let password = env::var("RUST_WEB_API_PASSWORD").expect("to have login password");
        let logins = RequestLoginUser::new(email, password);

        // instantiate a http client
        let http_client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .build()?;

        Ok(TestSetup {
            http_client,
            logins,
        })
    }
}
