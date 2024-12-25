use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use chrono::Utc;
use serde::Deserialize;
use crate::security;
use jsonwebtoken::{encode, Header, EncodingKey};
#[derive(Debug)]
#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String
}

#[derive(sqlx::FromRow)]
struct UsersRecord {
    password: String
}

pub async fn login(data: web::Json<LoginData>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    println!("{:?}", data.username);
    let user_result = sqlx::query_as::<_, UsersRecord>(
        "SELECT password FROM users WHERE username=$1",
    )
    .bind(data.username.clone())
    .fetch_one(pool.get_ref())
    .await;
    
    let user_record = match user_result {
        Ok(record) => record,    
        Err(e) => return HttpResponse::BadRequest().json(format!("User not found. {}", e)),
    };
    
    let is_valid = match verify(&data.password, &user_record.password) {
        Ok(valid) => valid,
        Err(_) => return HttpResponse::InternalServerError().json("Password could not be verified.")
    };
    
    if !is_valid { return HttpResponse::BadRequest().json("Incorrect password!"); }
    
    let expiration = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(3600))
    .expect("Timestamp should be valid")
    .timestamp() as usize;
    
    let claims = security::Claims {
        sub: data.username.clone(),
        exp: expiration
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to create token")
    };
    
    HttpResponse::Ok().json(token)
    
}