use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

mod encrypt;

mod routes {
    pub mod signup;
    pub mod login;
    pub mod test;
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
            .service(web::resource("/signup").route(web::post().to(routes::signup::signup)))
            .service(web::resource("/login").route(web::post().to(routes::login::login)))
            .service(web::resource("/test").route(web::get().to(routes::test::test_protected)))
    })
    
    .bind(("127.0.0.1", 4000))?.run().await
}