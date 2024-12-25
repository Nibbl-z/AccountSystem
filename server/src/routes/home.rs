use std::{fs::File, io::Read};
use base64::{prelude::BASE64_STANDARD, Engine};
use actix_web::{HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::security::authorization;

pub async fn home(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization");
    
    if let Some(auth_header) = auth_header {
        match authorization(auth_header) {
            Some(token_data) => {
                let mut file = File::open("./img/goose.png").unwrap();
                let mut byte_buffer = Vec::new();
                file.read_to_end(&mut byte_buffer).unwrap();
                
                let goose = BASE64_STANDARD.encode(&byte_buffer);
                
                return HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                    "username": token_data.claims.sub,
                    "goose": goose
                }))
            }
            None => return HttpResponse::Unauthorized().body("Authorization was unsuccessful")
        }
    }
    
    return HttpResponse::Unauthorized().body("Authorization token missing");
}