use std::process::exit;
use rand::{
    Rng, thread_rng,
    distributions::Alphanumeric
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, Error
    }, Argon2};
use crate::{
    models::{
        util::HashResult,
        api::ApiKey
    },
};

pub fn hash_string(raw_string: Option<String>) -> Result<HashResult, Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash =
        argon2.hash_password(raw_string.unwrap().as_ref(), &salt)?.to_string();

    let password = HashResult {
        hash: password_hash,
        salt: salt.to_string()
    };

    Ok(password)
}

pub fn gen_api_key() {
    let new_key_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    print!("{}",
        new_key_string
    );
    exit(0)
}

