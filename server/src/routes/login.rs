use actix_web::{web, HttpResponse, Responder};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Utc;
use serde::Deserialize;
use crate::encrypt;
use jsonwebtoken::{encode, Header, EncodingKey};
#[derive(Debug)]
#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String
}

#[derive(sqlx::FromRow)]
struct UsersRecord {
    nonce: String,
    password: String
}

pub async fn login(data: web::Json<LoginData>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    println!("{:?}", data.username);
    let user_result = sqlx::query_as::<_, UsersRecord>(
        "SELECT nonce, password FROM users WHERE username=$1",
    )
    .bind(data.username.clone())
    .fetch_one(pool.get_ref())
    .await;
    
    let user_record = match user_result {
        Ok(record) => record,
        
        Err(e) => return HttpResponse::BadRequest().json(format!("User not found. {}", e)),
    };
    
    let nonce_vec = match BASE64_STANDARD.decode(&user_record.nonce) {
        Ok(nonce) => nonce,
        Err(_) => return HttpResponse::InternalServerError().json("(Database error) Nonce could not be decoded."),
    };
    
    let encrypted_password = encrypt::encrypt_with_nonce(&data.password, &nonce_vec);
    let password_base64 = BASE64_STANDARD.encode(encrypted_password);
    
    if user_record.password != password_base64 {
        return HttpResponse::BadRequest().json("Incorrect password!");
    }
    
    let expiration = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(3600))
    .expect("Timestamp should be valid")
    .timestamp() as usize;
    
    let claims = encrypt::Claims {
        sub: data.username.clone(),
        exp: expiration
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to create token")
    };
    
    HttpResponse::Ok().json(token)
    
}