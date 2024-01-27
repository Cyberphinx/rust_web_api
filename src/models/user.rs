// use axum::{http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

// use crate::utilities::{
//     app_error::AppError,
//     hash::{hash_password, verify_password},
//     jwt::create_token,
//     token_wrapper::TokenWrapper,
// };

/// Domain Entity for User in the database
#[derive(Serialize, Deserialize, Clone, Default, FromRow)]
pub struct User {
    pub id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub image: Option<String>,
    pub role: String,
    pub description: Option<String>,
    pub token: Option<String>,
}

/// DTO (Data Transfer Object) - User HTTP JSON Response object
#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

/// DTO (Data Transfer Object) - User HTTP Response object
#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub email: String,
    pub role: String,
    pub token: String,
}

/// DTO (Data Transfer Object) - User Signup HTTP Request object
#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub email: String,
    pub password: String,
}

/// DTO (Data Transfer Object) - User Login HTTP Request object
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestLoginUser {
    pub email: String,
    pub password: String,
}

/// constructor function to create a new instance of RequestLoginUser
impl RequestLoginUser {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

// Implementation for User Entity
// impl User {
//     pub async fn create_user(
//         pool: Pool<Postgres>,
//         jwt_secret: TokenWrapper,
//         request_user: RequestCreateUser,
//         role: String,
//     ) -> Result<Json<ResponseDataUser>, AppError> {
//         let new_user = sqlx::query!(
//             "INSERT INTO users (email, password, role) VALUES ($1, $2, $3) RETURNING id;",
//             request_user.email,
//             hash_password(&request_user.password)?,
//             role,
//         )
//         .fetch_one(&pool)
//         .await?;

//         let token = create_token(&jwt_secret.0, new_user.id, role.clone())?;
//         let updated_user = sqlx::query_as!(
//             User,
//             "UPDATE users SET token = $1 WHERE id = $2 RETURNING *;",
//             token,
//             new_user.id
//         )
//         .fetch_one(&pool)
//         .await?;

//         let response_user = ResponseUser {
//             id: updated_user.id,
//             email: updated_user.email,
//             role,
//             token,
//         };

//         Ok(Json(ResponseDataUser {
//             data: response_user,
//         }))
//     }

//     pub async fn login(
//         pool: Pool<Postgres>,
//         jwt_secret: TokenWrapper,
//         request_user: RequestLoginUser,
//     ) -> Result<Json<ResponseDataUser>, AppError> {
//         let user = sqlx::query_as!(
//             User,
//             "SELECT * FROM users WHERE email = $1;",
//             request_user.email
//         )
//         .fetch_one(&pool)
//         .await;
//         match user {
//             Ok(user) => {
//                 if !verify_password(&request_user.password, &user.password)? {
//                     AppError::new(StatusCode::UNAUTHORIZED, "incorrect email and/or password");
//                 }
//                 let token = create_token(&jwt_secret.0, user.id, user.role)?;
//                 let updated_user = sqlx::query!(
//                     "UPDATE users SET token = $1 WHERE id = $2 RETURNING *;",
//                     token,
//                     user.id
//                 )
//                 .fetch_one(&pool)
//                 .await?;

//                 let response_user = ResponseUser {
//                     id: updated_user.id,
//                     email: updated_user.email,
//                     role: updated_user.role,
//                     token: updated_user.token.unwrap(),
//                 };

//                 Ok(Json(ResponseDataUser {
//                     data: response_user,
//                 }))
//             }
//             Err(error) => {
//                 eprintln!("{}", error);
//                 Err(AppError::new(
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     "Something went wrong",
//                 ))
//             }
//         }
//     }

//     pub async fn logout(pool: Pool<Postgres>, user: User) -> Result<StatusCode, AppError> {
//         sqlx::query!(
//             "UPDATE users SET token = $1 WHERE id = $2;",
//             None::<String>,
//             user.id
//         )
//         .execute(&pool)
//         .await?;
//         Ok(StatusCode::OK)
//     }
// }
