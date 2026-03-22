use crate::error::Result;
use std::collections::HashMap;

pub mod argon2;
pub mod bcrypt;
pub mod classic;
pub mod pbkdf2;
pub mod scrypt;

use crate::hasher::argon2::{Argon2dHasher, Argon2iHasher, Argon2idHasher};
use crate::hasher::bcrypt::BcryptHasher;
use crate::hasher::classic::{Md5Hasher, Sha256Hasher, Sha512Hasher};
use crate::hasher::pbkdf2::Pbkdf2Hasher;
use crate::hasher::scrypt::ScryptHasher;

/// Trait representing a password hasher.
pub trait Hasher {
    /// Hashes a password with the given configuration.
    fn hash(&self, password: &[u8], config: &HasherConfig) -> Result<String>;

    /// Verifies a password against a hash.
    fn verify(&self, password: &[u8], hash: &str) -> Result<bool>;

    /// Returns the name of the algorithm.
    fn name(&self) -> &'static str;
}

/// Configuration for various hashing algorithms.
#[derive(Debug, Clone)]
pub enum HasherConfig {
    Argon2i {
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
    },
    Argon2d {
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
    },
    Argon2id {
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
    },
    Bcrypt {
        cost: u32,
    },
    Scrypt {
        n: u32,
        r: u32,
        p: u32,
    },
    Pbkdf2 {
        iterations: u32,
    },
    Sha256,
    Sha512,
    Md5,
}

/// Registry for hashers.
pub struct HasherRegistry {
    hashers: HashMap<String, Box<dyn Hasher + Send + Sync>>,
}

impl HasherRegistry {
    pub fn new() -> Self {
        Self {
            hashers: HashMap::new(),
        }
    }

    pub fn register(&mut self, hasher: Box<dyn Hasher + Send + Sync>) {
        self.hashers.insert(hasher.name().to_string(), hasher);
    }

    pub fn get(&self, name: &str) -> Option<&(dyn Hasher + Send + Sync)> {
        self.hashers.get(name).map(|h| h.as_ref())
    }

    pub fn list_algorithms(&self) -> Vec<String> {
        self.hashers.keys().cloned().collect()
    }

    pub fn all_hashers(&self) -> Vec<&(dyn Hasher + Send + Sync)> {
        self.hashers.values().map(|h| h.as_ref()).collect()
    }
}

impl Default for HasherRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(Argon2iHasher));
        registry.register(Box::new(Argon2dHasher));
        registry.register(Box::new(Argon2idHasher));
        registry.register(Box::new(BcryptHasher));
        registry.register(Box::new(ScryptHasher));
        registry.register(Box::new(Pbkdf2Hasher));
        registry.register(Box::new(Sha256Hasher));
        registry.register(Box::new(Sha512Hasher));
        registry.register(Box::new(Md5Hasher));
        registry
    }
}
