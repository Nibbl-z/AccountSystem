use std::{fs::File, io::Read};
use actix_web::{HttpResponse, Responder, HttpRequest};
use crate::security::authorization;

pub async fn home(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization");
    
    if let Some(auth_header) = auth_header {
        match authorization(auth_header) {
            Some(token_data) => {
                let mut file = File::open("./img/goose.png").unwrap();
                let mut byte_buffer = Vec::new();
                file.read_to_end(&mut byte_buffer).unwrap();
                
                return HttpResponse::Ok()
                .content_type("image/png")
                .insert_header(("X-Username", token_data.claims.sub))
                .insert_header(("Access-Control-Expose-Headers", "X-Username"))
                .body(byte_buffer);
            }
            None => return HttpResponse::Unauthorized().body("Authorization was unsuccessful")
        }
    }
    
    return HttpResponse::Unauthorized().body("Authorization token missing");
}