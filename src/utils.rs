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

pub fn hash_string_with_salt(raw_string: String, salt_string: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::encode_b64(salt_string.as_ref()).unwrap();

    argon2.hash_password(raw_string.as_ref(), &salt).unwrap().to_string()
}


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

pub fn gen_api_key(permission_level: i8) {
    let new_key_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let new_secret_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let key_hash: String = hash_string_with_salt(
        new_key_string.clone(), new_secret_string.clone()
    );

    println!("{}",
        format!(
            "\n\
            +------------------------- API Key pair generated -------------------------+\n\
            |                                                                          |\n\
            | Key: {}    |\n\
            | Secret: {}                                 |\n\
            |                                                                          |\n\
            +--------------------------------------------------------------------------+",
            new_key_string, new_secret_string
        )
    );

    exit(0)
}

