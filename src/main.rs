use std::io::Read;
use k256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret};

use rand_core::OsRng;


fn main() {

    let a = EphemeralSecret::random(&mut OsRng);
    let A = EncodedPoint::from(a.public_key());

    let b = EphemeralSecret::random(&mut OsRng);
    let B = EncodedPoint::from(b.public_key());


    let bob_public = PublicKey::from_sec1_bytes(B.as_ref()).expect("Bob's public key invalid");


    let alice_public = PublicKey::from_sec1_bytes(A.as_ref()).expect("Alice's public key invalid");

    let Abytes= A.as_ref();
    println!("\nAlice public key {:x?}", Abytes.bytes());
    println!("\nAlice public key {:x?}",hex::encode(Abytes));

    let Bbytes= B.as_ref();
    println!("\nBob public key {:x?}",hex::encode(Bbytes));


    let alice_shared = a.diffie_hellman(&bob_public);
    let bob_shared = b.diffie_hellman(&alice_public);

    let shared1= alice_shared.raw_secret_bytes();
    println!("\nAlice shared key {:x?}",hex::encode(shared1));
    println!("\nAlice shared key {:x?}",shared1.bytes());

    let shared2= bob_shared.raw_secret_bytes();
    println!("\nBob shared key {:x?}",hex::encode(shared2));


    if alice_shared.raw_secret_bytes()==bob_shared.raw_secret_bytes(){
        println!("\nKeys are the same")
    }

}