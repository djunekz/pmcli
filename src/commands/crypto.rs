use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use base64::{encode, decode};

pub fn encrypt(data: &str, password: &str) -> String {
    let key = Key::<Aes256Gcm>::from_slice(&sha256(password));
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), data.as_bytes())
        .unwrap();

    format!(
        "{}:{}",
        encode(nonce_bytes),
        encode(ciphertext)
    )
}

pub fn decrypt(data: &str, password: &str) -> String {
    let parts: Vec<&str> = data.split(':').collect();
    let nonce = decode(parts[0]).unwrap();
    let ciphertext = decode(parts[1]).unwrap();

    let key = Key::<Aes256Gcm>::from_slice(&sha256(password));
    let cipher = Aes256Gcm::new(key);

    let plain = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .unwrap();

    String::from_utf8(plain).unwrap()
}

fn sha256(input: &str) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}
