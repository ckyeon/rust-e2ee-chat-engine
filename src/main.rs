use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng;
use std::io::Read;
mod crypto;

fn main() {
    println!("Hello, Rust e2ee chat engine!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffie_hellman() {
        let a = EphemeralSecret::random(&mut OsRng);
        let A = EncodedPoint::from(a.public_key());

        let b = EphemeralSecret::random(&mut OsRng);
        let B = EncodedPoint::from(b.public_key());

        let bob_public = PublicKey::from_sec1_bytes(B.as_ref()).expect("Bob's public key invalid");

        let alice_public = PublicKey::from_sec1_bytes(A.as_ref()).expect("Alice's public key invalid");

        let alice_shared = a.diffie_hellman(&bob_public);
        let bob_shared = b.diffie_hellman(&alice_public);

        assert_eq!(alice_shared.raw_secret_bytes(), bob_shared.raw_secret_bytes());
    }
}
