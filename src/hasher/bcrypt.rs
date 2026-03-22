use crate::error::{Result, SaltError};
use crate::hasher::{Hasher, HasherConfig};
use bcrypt::{hash, verify};

pub struct BcryptHasher;

impl Hasher for BcryptHasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if let HasherConfig::Bcrypt { cost } = config {
            hash(password, *cost)
                .map_err(|e| SaltError::HashingError(e.to_string()))
        } else {
            Err(SaltError::InvalidParameter("Invalid config for bcrypt".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        verify(password, hash)
            .map_err(|e| SaltError::VerificationError(e.to_string()))
    }

    fn name(&self) -> &'static str {
        "bcrypt"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcrypt_hash_verify() {
        let hasher = BcryptHasher;
        let password = b"password123";
        let config = HasherConfig::Bcrypt { cost: 4 };

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }
}
