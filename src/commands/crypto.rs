use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::password_hash::rand_core::OsRng as SaltRng;
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;
use base64::{engine::general_purpose::STANDARD, Engine};

fn derive_key(password: &str, salt: &SaltString) -> Key<Aes256Gcm> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), salt)
        .expect("Key derivation failed");
    let output = hash.hash.expect("Missing derived key");
    Key::<Aes256Gcm>::clone_from_slice(output.as_bytes())
}

pub fn encrypt(data: &str, password: &str) -> String {
    let salt = SaltString::generate(&mut SaltRng);
    let cipher = Aes256Gcm::new(&derive_key(password, &salt));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, data.as_bytes()).unwrap();

    format!(
        "{}:{}:{}",
        salt.as_str(),
        STANDARD.encode(nonce),
        STANDARD.encode(ciphertext)
    )
}

pub fn decrypt(data: &str, password: &str) -> String {
    let mut parts = data.splitn(3, ':');
    let salt_part = parts.next().expect("Invalid encrypted payload");
    let nonce_part = parts.next().expect("Invalid encrypted payload");
    let cipher_part = parts.next().expect("Invalid encrypted payload");

    let salt = SaltString::from_b64(salt_part).expect("Invalid salt");
    let nonce_bytes = STANDARD.decode(nonce_part).unwrap();
    let ciphertext = STANDARD.decode(cipher_part).unwrap();

    let cipher = Aes256Gcm::new(&derive_key(password, &salt));
    let plain = cipher
        .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref())
        .unwrap();

    String::from_utf8(plain).unwrap()
}
