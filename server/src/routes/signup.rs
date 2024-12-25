use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use bcrypt::{DEFAULT_COST, hash};

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
    
    let hashed_password = hash(&data.password, DEFAULT_COST);
    
    match hashed_password {
        Ok(_) => {}
        Err(_) => return HttpResponse::InternalServerError().body("Password failed to hash")
    }
    
    let result = sqlx::query(
        "INSERT INTO users (username, password) VALUES ($1, $2)"
    )
    .bind(data.username.clone())
    .bind(hashed_password.unwrap())
    .execute(pool.get_ref())
    .await;
    
    match result {
        Ok (_) => HttpResponse::Ok().body(format!("{} has signed up successfully!", data.username)),
        Err (_) => HttpResponse::InternalServerError().body("Database error.")
    }
}