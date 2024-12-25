use actix_web::http::header::HeaderValue;

use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn authorization(header : &HeaderValue) -> Option<TokenData<Claims>> {
    let auth_str = header.to_str().unwrap();
    if auth_str.starts_with("Bearer ") {
        let token = &auth_str[7..];
        let validation = Validation::default();
        let decoding_key = DecodingKey::from_secret("secret".as_ref());

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => Some(token_data),
            Err(_) => None
        }
    } else {
        None
    }
}