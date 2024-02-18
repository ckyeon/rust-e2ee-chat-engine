// src/my_module/mod.rs

mod salt;
mod keypair;
mod core;
mod shared_key;

pub use salt::generate_salt;

pub use keypair::generate_random_keypair;

pub use shared_key::generate_shared_key;