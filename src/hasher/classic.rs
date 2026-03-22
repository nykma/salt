use crate::error::{Result, SaltError};
use crate::hasher::{Hasher, HasherConfig};
use sha2::{Sha256, Sha512, Digest};

pub struct Sha256Hasher;
pub struct Sha512Hasher;
pub struct Md5Hasher;

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn from_hex(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect()
}

impl Hasher for Sha256Hasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if matches!(config, HasherConfig::Sha256) {
            let salt: [u8; 16] = rand::random();
            let mut hasher = Sha256::new();
            hasher.update(&salt);
            hasher.update(password);
            let result = hasher.finalize();
            Ok(format!("{}:{}", to_hex(&salt), to_hex(&result)))
        } else {
            Err(SaltError::InvalidParameter("Invalid config for SHA-256".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parts: Vec<&str> = hash.split(':').collect();
        if parts.len() != 2 {
            return Ok(false);
        }
        let salt = match from_hex(parts[0]) {
            Some(s) => s,
            None => return Ok(false),
        };
        let original_hash = match from_hex(parts[1]) {
            Some(h) => h,
            None => return Ok(false),
        };

        let mut hasher = Sha256::new();
        hasher.update(&salt);
        hasher.update(password);
        let result = hasher.finalize();

        Ok(result.as_slice() == original_hash.as_slice())
    }

    fn name(&self) -> &'static str {
        "sha256"
    }
}

impl Hasher for Sha512Hasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if matches!(config, HasherConfig::Sha512) {
            let salt: [u8; 16] = rand::random();
            let mut hasher = Sha512::new();
            hasher.update(&salt);
            hasher.update(password);
            let result = hasher.finalize();
            Ok(format!("{}:{}", to_hex(&salt), to_hex(&result)))
        } else {
            Err(SaltError::InvalidParameter("Invalid config for SHA-512".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parts: Vec<&str> = hash.split(':').collect();
        if parts.len() != 2 {
            return Ok(false);
        }
        let salt = match from_hex(parts[0]) {
            Some(s) => s,
            None => return Ok(false),
        };
        let original_hash = match from_hex(parts[1]) {
            Some(h) => h,
            None => return Ok(false),
        };

        let mut hasher = Sha512::new();
        hasher.update(&salt);
        hasher.update(password);
        let result = hasher.finalize();

        Ok(result.as_slice() == original_hash.as_slice())
    }

    fn name(&self) -> &'static str {
        "sha512"
    }
}

impl Hasher for Md5Hasher {
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String> {
        if matches!(config, HasherConfig::Md5) {
            let salt: [u8; 16] = rand::random();
            let mut combined = Vec::with_capacity(salt.len() + password.len());
            combined.extend_from_slice(&salt);
            combined.extend_from_slice(password);
            let result = md5::compute(&combined);
            Ok(format!("{}:{:x}", to_hex(&salt), result))
        } else {
            Err(SaltError::InvalidParameter("Invalid config for MD5".to_string()))
        }
    }

    fn verify(&self, password: &[u8], hash: &str) -> Result<bool> {
        let parts: Vec<&str> = hash.split(':').collect();
        if parts.len() != 2 {
            return Ok(false);
        }
        let salt = match from_hex(parts[0]) {
            Some(s) => s,
            None => return Ok(false),
        };
        let original_hash = match from_hex(parts[1]) {
            Some(h) => h,
            None => return Ok(false),
        };

        let mut combined = Vec::with_capacity(salt.len() + password.len());
        combined.extend_from_slice(&salt);
        combined.extend_from_slice(password);
        let result = md5::compute(&combined);

        Ok(result.0 == *original_hash.as_slice())
    }

    fn name(&self) -> &'static str {
        "md5"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash_verify() {
        let hasher = Sha256Hasher;
        let password = b"password123";
        let config = HasherConfig::Sha256;

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_sha512_hash_verify() {
        let hasher = Sha512Hasher;
        let password = b"password123";
        let config = HasherConfig::Sha512;

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_md5_hash_verify() {
        let hasher = Md5Hasher;
        let password = b"password123";
        let config = HasherConfig::Md5;

        let hash = hasher.hash(password, &config).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
        assert!(!hasher.verify(b"wrong_password", &hash).unwrap());
    }
}
