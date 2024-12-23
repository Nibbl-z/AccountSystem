use actix_web::{web, HttpResponse, Responder};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::Deserialize;

use crate::encrypt;

#[derive(Deserialize)]
pub struct SignupData {
    username: String,
    password: String
}

pub async fn signup(data: web::Json<SignupData>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let find_username = sqlx::query(
        "SELECT username FROM users WHERE LOWER(username)=$1",
    )
    .bind(data.username.to_lowercase())
    .fetch_optional(pool.get_ref())
    .await;
    
    match find_username {
        Ok(Some(_)) => return HttpResponse::BadRequest().body("Username is already taken!"),
        Ok(None) => {},
        Err(_) => return HttpResponse::InternalServerError().body("Database error")
    }

    let (encrypted_password, nonce) = encrypt::encrypt(&data.password);
    let password_base64 = BASE64_STANDARD.encode(encrypted_password);
    let nonce_base64 = BASE64_STANDARD.encode(nonce);
    println!("{}, {}", data.username, &password_base64);    
    
    let result = sqlx::query(
        "INSERT INTO users (username, password, nonce) VALUES ($1, $2, $3)"
    )
    .bind(data.username.clone())
    .bind(password_base64)
    .bind(nonce_base64).execute(pool.get_ref())
    .await;
    
    match result {
        Ok (_) => HttpResponse::Ok().body(format!("{} has signed up successfully!", data.username)),
        Err (_) => HttpResponse::InternalServerError().body("Database error.")
    }
}