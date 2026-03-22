use crate::error::{Result, SaltError};
use crate::hasher::{Hasher, HasherConfig};
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt, Params
};

pub struct ScryptHasher;

impl Hasher for ScryptHasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if let HasherConfig::Scrypt { n, r, p } = config {
            if *n <= 1 || !n.is_power_of_two() {
                return Err(SaltError::InvalidParameter("scrypt 'n' must be a power of 2 and greater than 1".to_string()));
            }
            let log_n = (*n as f64).log2() as u8;
            let params = Params::new(log_n, *r, *p, Params::RECOMMENDED_LEN)
                .map_err(|e| SaltError::HashingError(e.to_string()))?;
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = Scrypt.hash_password_customized(password, None, None, params, &salt)
                .map_err(|e| SaltError::HashingError(e.to_string()))?
                .to_string();
            Ok(password_hash)
        } else {
            Err(SaltError::InvalidParameter("Invalid config for scrypt".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| SaltError::VerificationError(e.to_string()))?;
        Ok(Scrypt.verify_password(password, &parsed_hash).is_ok())
    }

    fn name(&self) -> &'static str {
        "scrypt"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrypt_hash_verify() {
        let hasher = ScryptHasher;
        let password = b"password123";
        // n = 1024 (log2(n) = 10)
        let config = HasherConfig::Scrypt { n: 1024, r: 8, p: 1 };

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }
}
