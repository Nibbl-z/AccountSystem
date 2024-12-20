use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

#[get("/")]
async fn hello(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let rows = sqlx::query!("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
        .expect("Failed to execute query");
    
    let mut response: String = String::from("There is: ");
    let mut index = 0;
    let length = rows.len();

    for row in rows {
        match row.name {
            Some(name) => {
                if index == length - 1 {
                    response.push_str(" AND LAST BUT NOT LEAST!!!: ");
                } else if index > 0 {
                    response.push_str(" AND ");
                }
                
                response.push_str(&name);
                
                
            },
            None => {}
        }

        index += 1;
    }

    HttpResponse::Ok().body(response)
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
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}