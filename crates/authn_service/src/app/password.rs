use argon2::{Algorithm, Argon2, Params, Version, password_hash::PasswordHasher};

use crate::errors::Error::{self, PasswordHashAction, PasswordHashContext};

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
