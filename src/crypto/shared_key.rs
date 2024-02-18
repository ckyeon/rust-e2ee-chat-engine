use rand_core::OsRng;
use k256::{EncodedPoint, ecdh::EphemeralSecret, PublicKey as K256PublicKey};
use crate::crypto::keypair::PublicKey;

pub struct SharedKey {
    // 32 bytes
    key: [u8; 32],
}

impl SharedKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.key
    }
}
pub fn generate_shared_key(bob_public_key: &PublicKey) -> (PublicKey, SharedKey) {
    let ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let encoded = EncodedPoint::from(ephemeral_secret.public_key());
    let alice_public_key = PublicKey::from_bytes(encoded.as_ref().try_into().expect("ephemeral secret's public key invalid"));
    let bob_k256_public_key = K256PublicKey::from_sec1_bytes(bob_public_key.to_bytes().as_ref().try_into().unwrap()).expect("Bob's public key invalid");
    let alice_shared = ephemeral_secret.diffie_hellman(&bob_k256_public_key);
    let shared_key = SharedKey {
        key: alice_shared.raw_secret_bytes().as_slice().try_into().unwrap()
    };
    (alice_public_key, shared_key)
}