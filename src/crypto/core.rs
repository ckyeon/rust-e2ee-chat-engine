use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv,
    Nonce, // Or `Aes128GcmSiv`
};

// add parameter nonce
pub fn encrypt(key: &[u8], plaintext: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = KeyInit::from_slice(key);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(nonce).unwrap(); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(nonce, plaintext).unwrap();
    ciphertext
}

pub fn decrypt(key: &[u8], ciphertext: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = KeyInit::from_slice(key);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(nonce).unwrap(); // 96-bits; unique per message
    let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
    plaintext
}
