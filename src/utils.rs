use std::process::exit;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, Error
    }, Argon2};
use crate::models::util::Password;

pub fn hash_password(password: Option<String>) -> Result<Password, Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash =
        argon2.hash_password(password.unwrap().as_ref(), &salt)?.to_string();

    let password = Password {
        hashed_password: password_hash,
        salt: salt.to_string()
    };

    Ok(password)
}

pub fn gen_api_key() {
    print!(
        r#"
        print an api key and store it :)
        "#
    );
    exit(0)
}

