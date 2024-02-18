use rand::rngs::OsRng;

pub struct PublicKey {
    // 33 bytes
    key: [u8; 33],
}

impl PublicKey {
    pub fn from_bytes(bytes: [u8; 33]) -> PublicKey {
        PublicKey {
            key: bytes
        }
    }
    pub fn to_bytes(&self) -> [u8; 33] {
        self.key
    }
}



pub struct PrivateKey {
    // 32 bytes
    key: [u8; 32],
}

impl PrivateKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.key
    }
}

pub fn generate_random_keypair() -> (PublicKey, PrivateKey) {
    let secret = k256::SecretKey::random(&mut OsRng);
    let secret_bytes = secret.to_bytes();
    let private_key = PrivateKey {
        key: secret_bytes.try_into().unwrap()
    };
    let public_key = PublicKey {
        key: secret.public_key().to_sec1_bytes().as_ref().try_into().unwrap(),
    };

    (public_key, private_key)
}

