use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{Error::PasswordInvalid, PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use crate::errors::Error::{self, InvalidCredentials, PasswordHashAction, PasswordHashContext};

pub async fn hash(pepper: &[u8], password: &[u8]) -> Result<String, Error> {
    let argon2 = Argon2::new_with_secret(
        pepper,
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .map_err(|_| PasswordHashContext)?;

    let hash_password = argon2.hash_password(password).map_err(|_| PasswordHashAction)?.to_string();

    Ok(hash_password)
}

pub async fn verify(pepper: &[u8], password: &[u8], password_hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| PasswordHashAction)?;

    let argon2 = Argon2::new_with_secret(
        pepper,
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )
    .map_err(|_| PasswordHashContext)?;

    let res = argon2.verify_password(password, &parsed_hash).map_err(|err| {
        if err == PasswordInvalid { InvalidCredentials } else { PasswordHashAction }
    });

    Ok(res.is_ok())
}
