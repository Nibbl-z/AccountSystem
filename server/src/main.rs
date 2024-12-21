use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignupData {
    username: String,
    password: String
}

#[post("/signup")]
async fn signup(data: web::Json<SignupData>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    println!("{}, {}", data.username, data.password);    
    
    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        data.username,
        data.password
    ).execute(pool.get_ref())
    .await;

    match result {
        Ok (_) => HttpResponse::Ok().body(format!("{} has signed up successfully!!!", data.username)),
        Err (_) => HttpResponse::InternalServerError().body("Something BAD happened.")
    }
}

#[get("/")]
async fn hello(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let rows = sqlx::query!("SELECT id, username, password FROM users")
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
    })
    
    .bind(("127.0.0.1", 4000))?.run().await
}