use rand::{distributions::Alphanumeric, Rng};

pub fn generate_salt(length: usize) -> String {
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    salt
}