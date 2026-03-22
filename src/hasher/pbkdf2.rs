use crate::error::{Result, SaltError};
use crate::hasher::{Hasher, HasherConfig};
use password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
};
use pbkdf2::Pbkdf2;

pub struct Pbkdf2Hasher;

impl Hasher for Pbkdf2Hasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if let HasherConfig::Pbkdf2 { iterations } = config {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = Pbkdf2.hash_password_customized(
                password,
                None,
                None,
                pbkdf2::Params {
                    rounds: *iterations,
                    output_length: 32,
                },
                &salt
            )
                .map_err(|e: password_hash::Error| SaltError::HashingError(e.to_string()))?
                .to_string();
            Ok(password_hash)
        } else {
            Err(SaltError::InvalidParameter("Invalid config for pbkdf2".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e: password_hash::Error| SaltError::VerificationError(e.to_string()))?;
        Ok(Pbkdf2.verify_password(password, &parsed_hash).is_ok())
    }

    fn name(&self) -> &'static str {
        "pbkdf2"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf2_hash_verify() {
        let hasher = Pbkdf2Hasher;
        let password = b"password123";
        let config = HasherConfig::Pbkdf2 { iterations: 1000 };

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }
}
