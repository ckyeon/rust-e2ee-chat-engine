use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex_literal::hex;
use cbc::{Cbc, Decryptor, Encryptor};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn encrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let mut encryptor = Aes128CbcEnc::new_from_slices(key, iv).unwrap();
    let mut buffer = data.to_vec();
    let padding = Pkcs7::pad(&mut buffer, 16).unwrap();
    encryptor.encrypt(&mut buffer, padding).unwrap();
    buffer
}

pub fn decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let mut decryptor = Aes128CbcDec::new_from_slices(key, iv).unwrap();
    let mut buffer = data.to_vec();
    decryptor.decrypt(&mut buffer).unwrap();
    let padding = Pkcs7::unpad(&mut buffer).unwrap();
    buffer.truncate(buffer.len() - padding);
    buffer
}