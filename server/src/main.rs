use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::{env, io::Read, string};
use dotenv::dotenv;
use serde::Deserialize;
use base64::prelude::*;

mod encrypt;

#[derive(Deserialize)]
struct SignupData {
    username: String,
    password: String
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String
}

#[post("/signup")]
async fn signup(data: web::Json<SignupData>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let find_username = sqlx::query!(
        "SELECT username FROM users WHERE LOWER(username)=$1",
        data.username.to_lowercase()
    )
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
    
    let result = sqlx::query!(
        "INSERT INTO users (username, password, nonce) VALUES ($1, $2, $3)",
        data.username,
        password_base64,
        nonce_base64
    ).execute(pool.get_ref())
    .await;

    match result {
        Ok (_) => HttpResponse::Ok().body(format!("{} has signed up successfully!", data.username)),
        Err (_) => HttpResponse::InternalServerError().body("Database error.")
    }
}

#[post("/login")]
async fn login(data: web::Json<LoginData>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let nonce_result = sqlx::query!(
        "SELECT nonce FROM users WHERE username=$1",
        data.username
    )
    .fetch_one(pool.get_ref())
    .await;
    
    let nonce_base64 = match nonce_result {
        Ok(record) => record.nonce,
        Err(_) => return HttpResponse::BadRequest().body("User not found."),
    };
    
    let nonce_vec = match BASE64_STANDARD.decode(&nonce_base64) {
        Ok(nonce) => nonce,
        Err(_) => return HttpResponse::InternalServerError().body("(Database error) Nonce could not be decoded."),
    };
    
    let encrypted_password = encrypt::encrypt_with_nonce(&data.password, &nonce_vec);
    let password_base64 = BASE64_STANDARD.encode(encrypted_password);
    
    let password_result = sqlx::query!(
        "SELECT password FROM users WHERE username=$1",
        data.username
    )
    .fetch_one(pool.get_ref())
    .await;
    
    let dbpassword_base64 = match password_result {
        Ok(password) => password.password,
        Err(_) => return HttpResponse::InternalServerError().body("(Database error) Password could not be found.")
    };

    if dbpassword_base64 == password_base64 {
        HttpResponse::Ok().body("Login successful!")
    } else {
        HttpResponse::BadRequest().body("Incorrect password!")
    }
    
    
}

#[get("/")]
async fn hello(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let rows = sqlx::query!("SELECT id, username, password, nonce FROM users")
    .fetch_all(pool.get_ref())
    .await
    .expect("Query failed");
    
    
    HttpResponse::Ok().body(format!("{:#?}", rows))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
        .expect("Failed to create pool");
    
    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
        )
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(signup)
            .service(login)
    })
    
    .bind(("127.0.0.1", 4000))?.run().await
}