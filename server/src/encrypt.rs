use aes_gcm::{
    aead::{consts::U12, generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};

use dotenv::dotenv;
use std::env;

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