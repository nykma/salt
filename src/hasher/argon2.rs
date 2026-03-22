use crate::error::{Result, SaltError};
use crate::hasher::{Hasher, HasherConfig};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Algorithm, Params
};

pub struct Argon2iHasher;
pub struct Argon2dHasher;
pub struct Argon2idHasher;

impl Hasher for Argon2iHasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if let HasherConfig::Argon2i { memory_cost, time_cost, parallelism } = config {
            let params = Params::new(*memory_cost, *time_cost, *parallelism, None)
                .map_err(|e| SaltError::HashingError(e.to_string()))?;
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::new(Algorithm::Argon2i, Default::default(), params);
            let password_hash = argon2.hash_password(password, &salt)
                .map_err(|e| SaltError::HashingError(e.to_string()))?
                .to_string();
            Ok(password_hash)
        } else {
            Err(SaltError::InvalidParameter("Invalid config for Argon2i".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| SaltError::VerificationError(e.to_string()))?;
        Ok(Argon2::default().verify_password(password, &parsed_hash).is_ok())
    }

    fn name(&self) -> &'static str {
        "argon2i"
    }
}

impl Hasher for Argon2dHasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if let HasherConfig::Argon2d { memory_cost, time_cost, parallelism } = config {
            let params = Params::new(*memory_cost, *time_cost, *parallelism, None)
                .map_err(|e| SaltError::HashingError(e.to_string()))?;
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::new(Algorithm::Argon2d, Default::default(), params);
            let password_hash = argon2.hash_password(password, &salt)
                .map_err(|e| SaltError::HashingError(e.to_string()))?
                .to_string();
            Ok(password_hash)
        } else {
            Err(SaltError::InvalidParameter("Invalid config for Argon2d".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| SaltError::VerificationError(e.to_string()))?;
        Ok(Argon2::default().verify_password(password, &parsed_hash).is_ok())
    }

    fn name(&self) -> &'static str {
        "argon2d"
    }
}

impl Hasher for Argon2idHasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if let HasherConfig::Argon2id { memory_cost, time_cost, parallelism } = config {
            let params = Params::new(*memory_cost, *time_cost, *parallelism, None)
                .map_err(|e| SaltError::HashingError(e.to_string()))?;
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::new(Algorithm::Argon2id, Default::default(), params);
            let password_hash = argon2.hash_password(password, &salt)
                .map_err(|e| SaltError::HashingError(e.to_string()))?
                .to_string();
            Ok(password_hash)
        } else {
            Err(SaltError::InvalidParameter("Invalid config for Argon2id".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| SaltError::VerificationError(e.to_string()))?;
        Ok(Argon2::default().verify_password(password, &parsed_hash).is_ok())
    }

    fn name(&self) -> &'static str {
        "argon2id"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2id_hash_verify() {
        let hasher = Argon2idHasher;
        let password = b"password123";
        let config = HasherConfig::Argon2id {
            memory_cost: 4096,
            time_cost: 1,
            parallelism: 1,
        };

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_argon2i_hash_verify() {
        let hasher = Argon2iHasher;
        let password = b"password123";
        let config = HasherConfig::Argon2i {
            memory_cost: 4096,
            time_cost: 1,
            parallelism: 1,
        };

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_argon2d_hash_verify() {
        let hasher = Argon2dHasher;
        let password = b"password123";
        let config = HasherConfig::Argon2d {
            memory_cost: 4096,
            time_cost: 1,
            parallelism: 1,
        };

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }
}
