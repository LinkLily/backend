use std::process::exit;
use serde::{Serialize, Deserialize};
use rand::{
    Rng, thread_rng,
    distributions::Alphanumeric
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, Error
    }, Argon2};

#[derive(Debug, Serialize, Deserialize)]
pub struct HashResult {
    pub hash: String,
    pub salt: String
}


pub fn hash_string_with_salt(raw_string: String, salt_string: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::encode_b64(salt_string.as_ref()).unwrap();

    argon2.hash_password(raw_string.as_ref(), &salt).unwrap().to_string()
}


pub fn hash_string(raw_string: String) -> Result<HashResult, Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash =
        argon2.hash_password(raw_string.as_ref(), &salt)?.to_string();

    let password = HashResult {
        hash: password_hash,
        salt: salt.to_string()
    };

    Ok(password)
}

pub fn validate_password(password: String) -> bool {
    let pass_strength =
        zxcvbn::zxcvbn(&password, &[]).unwrap();

    if pass_strength.score() < 3 {
        false
    } else {
        true
    }
}


// This does nothing for now & may be removed in the future

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

    // let key_hash: String = hash_string_with_salt(
    //     new_key_string.clone(), new_secret_string.clone()
    // );




    println!("{}",
        format!(
            "\n\
            +------------------------- API Key pair generated -------------------------+\n\
            |                                                                          |\n\
            | Key: {}    |\n\
            | Secret: {}                                 |\n\
            | Permission Level: {}                                                      |\n\
            |                                                                          |\n\
            +--------------------------------------------------------------------------+",
            new_key_string, new_secret_string, permission_level.to_string()
        )
    );

    exit(0)
}

