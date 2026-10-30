use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{engine::general_purpose::STANDARD, Engine};
use sha2::{Digest, Sha256};

fn derive_key(password: &str) -> Key<Aes256Gcm> {
    Sha256::digest(password.as_bytes())
}

pub fn encrypt(data: &str, password: &str) -> String {
    let cipher = Aes256Gcm::new(&derive_key(password));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, data.as_bytes()).unwrap();

    format!("{}:{}", STANDARD.encode(nonce), STANDARD.encode(ciphertext))
}

pub fn decrypt(data: &str, password: &str) -> String {
    let (nonce_part, cipher_part) = data.split_once(':').expect("Invalid encrypted payload");
    let nonce_bytes = STANDARD.decode(nonce_part).unwrap();
    let ciphertext = STANDARD.decode(cipher_part).unwrap();

    let cipher = Aes256Gcm::new(&derive_key(password));
    let plain = cipher
        .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref())
        .unwrap();

    String::from_utf8(plain).unwrap()
}
