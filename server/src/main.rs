use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

mod encrypt;

mod routes {
    pub mod signup;
    pub mod login;
    pub mod home;
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hai! This is the backend.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").expect("Port should be set").parse::<u16>().expect("PORT should be a valid u16");
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
            .service(root)
            .service(web::resource("/api/signup").route(web::post().to(routes::signup::signup)))
            .service(web::resource("/api/login").route(web::post().to(routes::login::login)))
            .service(web::resource("/api/home").route(web::get().to(routes::home::home)))
    })
    
    .bind(("localhost", port))?.run().await
}