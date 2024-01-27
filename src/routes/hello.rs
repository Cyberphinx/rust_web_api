pub struct HelloRouter;

impl HelloRouter {
    pub async fn hello_world() -> String {
        "Hello world from sanctum axum!".to_owned()
    }

    pub async fn hello_auth() -> String {
        "Hello world from authenticated sanctum axum!".to_owned()
    }

    pub async fn hello_admin() -> String {
        "Hello admin from authenticated sanctum axum!".to_owned()
    }
}
