use actix_web::http::header::HeaderValue;
use aes_gcm::{
    aead::{consts::U12, generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};

use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encrypt(input: &str) -> (Vec<u8>, Vec<u8>) {
    dotenv().ok();
    let key_str = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypted_data = cipher.encrypt(&nonce, input.as_bytes())
        .expect("failed to encrypt");
    
    (encrypted_data, nonce.to_vec())
}

pub fn encrypt_with_nonce(input: &str, nonce: &Vec<u8>) -> Vec<u8> {
    dotenv().ok();
    let key_str = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    
    let cipher = Aes256Gcm::new(&key);
    let nonce: &GenericArray<u8, U12> = Nonce::from_slice(&nonce);

    let encrypted_data = cipher.encrypt(&nonce, input.as_bytes())
        .expect("Failed to encrypt");
    
    encrypted_data
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