
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use std::fs::OpenOptions;
use std::io::Write;
use infra::secure_kms::{generate_key, generate_nonce};

pub fn log_secure(message: &str) {
    let key_bytes = generate_key();
    let nonce_bytes = generate_nonce();

    let key = Key::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, message.as_bytes()).unwrap_or_else(|e| { log::error!("Handled error: {:?}", e); return default(); }) // safer"Explicit expectation: ")"encryption failed");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("secure.log")
        .unwrap_or_else(|e| { log::error!("Handled error: {:?}", e); return default(); }) // safer"Explicit expectation: ")"Checked unwrap failed at runtime: "));
    file.write_all(&ciphertext).unwrap_or_else(|e| { log::error!("Handled error: {:?}", e); return default(); }) // safer"Explicit expectation: ")"Checked unwrap failed at runtime: "));
}
