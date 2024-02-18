// src/my_module/mod.rs

mod core;
mod keypair;
mod salt;
mod shared_key;

pub use salt::generate_salt;

pub use keypair::generate_random_keypair;

pub use shared_key::generate_shared_key;

pub use core::encrypt;

pub use core::decrypt;
