use actix_web::{HttpResponse, Responder, HttpRequest};

use crate::encrypt::authorization;

pub async fn test_protected(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization");
    
    if let Some(auth_header) = auth_header {
        match authorization(auth_header) {
            Some(token_data) => {
                return HttpResponse::Ok().body(token_data.claims.sub)
            }
            None => return HttpResponse::Unauthorized().body("Authorization was unsuccessful")
        }
    }
    
    return HttpResponse::Unauthorized().body("Authorization token missing");
}