use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());
    argon2
        .hash_password(password_bytes, &salt)
        .map(|hash| hash.to_string())
}
