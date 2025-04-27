use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());
    argon2
        .hash_password(password_bytes, &salt)
        .map(|hash| hash.to_string())
}

pub(crate) fn validate_password(
    password: &String,
    hash: &String,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = argon2::password_hash::PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &parsed_hash)
}

#[cfg(test)]
mod tests {
    use crate::features::auth::utils::password::hash_password;

    #[test]
    fn test_hash_password() {
        let password = "my_secure_password";
        let hashed_password = hash_password(password).unwrap();
        assert!(!hashed_password.is_empty());
        assert!(hashed_password.starts_with("$argon2id$"));
    }

    #[test]
    fn test_hash_password_empty() {
        let password = "";
        let hashed_password = hash_password(password).unwrap();
        assert!(!hashed_password.is_empty());
        assert!(hashed_password.starts_with("$argon2id$"));
    }

    #[test]
    fn test_hash_password_special_characters() {
        let password = "!@#$%^&*()_+";
        let hashed_password = hash_password(password).unwrap();
        assert!(!hashed_password.is_empty());
        assert!(hashed_password.starts_with("$argon2id$"));
    }
}
