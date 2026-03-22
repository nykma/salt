use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use clap::ValueEnum;

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Plain,
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashResult {
    pub password_hash: String,
    pub algorithm: String,
    pub salt: Option<String>,
    pub version: Option<String>,
    pub parameters: Option<HashMap<String, String>>,
    pub timestamp: String,
}

impl HashResult {
    pub fn new(password_hash: String, algorithm: String, salt: Option<String>) -> Self {
        use chrono::Utc;
        Self {
            password_hash,
            algorithm,
            salt,
            version: None,
            parameters: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn from_hash(hash: &str) -> Self {
        use password_hash::PasswordHash;
        use chrono::Utc;
        
        if let Ok(ph) = PasswordHash::new(hash) {
            let mut parameters = HashMap::new();
            for (key, value) in ph.params.iter() {
                let mapped_key = match key.as_str() {
                    "m" => "memory_cost",
                    "t" => "time_cost",
                    "p" => "parallelism",
                    "i" => "iterations",
                    "l" => "length",
                    "ln" => "cost", // scrypt uses ln (log2(n))
                    "r" => "r",
                    "p_scrypt" => "p", // to avoid confusion if needed, but scrypt also uses p
                    _ => key.as_str(),
                };
                parameters.insert(mapped_key.to_string(), value.to_string());
            }
            
            return Self {
                password_hash: hash.to_string(),
                algorithm: ph.algorithm.to_string(),
                salt: ph.salt.map(|s| s.to_string()),
                version: ph.version.map(|v| v.to_string()),
                parameters: if parameters.is_empty() { None } else { Some(parameters) },
                timestamp: Utc::now().to_rfc3339(),
            };
        }

        // Fallback for bcrypt or classic formats
        let mut algorithm = "unknown".to_string();
        let mut salt = None;
        let mut version = None;
        let mut parameters = HashMap::new();

        if hash.starts_with("$2") {
            algorithm = "bcrypt".to_string();
            let parts: Vec<&str> = hash.split('$').collect();
            if parts.len() >= 4 {
                version = Some(parts[1].to_string());
                parameters.insert("cost".to_string(), parts[2].to_string());
                if parts[3].len() >= 22 {
                    salt = Some(parts[3][..22].to_string());
                }
            }
        } else if hash.contains(':') {
            let parts: Vec<&str> = hash.split(':').collect();
            if parts.len() == 2 {
                salt = Some(parts[0].to_string());
                algorithm = "classic".to_string();
            }
        }

        Self {
            password_hash: hash.to_string(),
            algorithm,
            salt,
            version,
            parameters: if parameters.is_empty() { None } else { Some(parameters) },
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
