use clap::{Parser, Subcommand};
use salt::{HasherRegistry, HasherConfig, OutputFormat, Result, SaltError, HashResult};
use std::collections::HashMap;
use std::io::{self, Read, IsTerminal};

#[derive(Parser)]
#[command(name = "salt")]
#[command(about = "A high-performance CLI tool for password hashing generation and verification", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a password hash
    #[command(alias = "g")]
    Generate {
        /// Password to hash (optional, secure methods like stdin or interactive prompt are preferred)
        #[arg(index = 1)]
        password: Option<String>,

        /// Hashing algorithm to use
        #[arg(short, long, default_value = "argon2id")]
        algorithm: String,

        /// Custom salt (not yet supported, will be ignored)
        #[arg(short, long)]
        salt: Option<String>,

        /// Number of iterations (for PBKDF2)
        #[arg(short, long)]
        iterations: Option<u32>,

        /// Work factor / cost (for bcrypt and scrypt)
        #[arg(short, long)]
        work_factor: Option<u32>,

        /// Memory cost in KB (for Argon2)
        #[arg(short, long)]
        memory: Option<u32>,

        /// Time cost / iterations (for Argon2)
        #[arg(short, long)]
        time: Option<u32>,

        /// Output format
        #[arg(short, long, default_value = "plain")]
        output: OutputFormat,
    },
    /// Verify a password against a hash
    #[command(alias = "v")]
    Verify {
        /// Password to verify (optional, secure methods like stdin or interactive prompt are preferred)
        #[arg(index = 1)]
        password: Option<String>,

        /// Hash to verify against (or the first argument if only one is provided)
        #[arg(index = 2)]
        hash: Option<String>,

        /// Output format
        #[arg(short, long, default_value = "plain")]
        output: OutputFormat,

        /// Show detailed verification info
        #[arg(short, long)]
        verbose: bool,
    },
    /// List all supported hashing algorithms
    #[command(alias = "a")]
    Algorithms {
        /// Output format
        #[arg(short, long, default_value = "plain")]
        output: OutputFormat,
    },
}

fn get_password(cli_password: Option<String>) -> Result<String> {
    if let Some(p) = cli_password {
        eprintln!("Warning: Providing password as a CLI argument is insecure. Use stdin or interactive prompt instead.");
        return Ok(p);
    }

    if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        return Ok(buffer.trim_end().to_string());
    }

    Ok(rpassword::prompt_password("Enter password: ")?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let registry = HasherRegistry::default();

    match cli.command {
        Commands::Generate {
            password: cli_password,
            algorithm,
            salt,
            iterations,
            work_factor,
            memory,
            time,
            output,
        } => {
            let password = get_password(cli_password)?;

            let hasher = registry.get(&algorithm).ok_or_else(|| {
                SaltError::InvalidAlgorithm(format!("Unknown algorithm: {}", algorithm))
            })?;

            let config = match algorithm.as_str() {
                "argon2id" => HasherConfig::Argon2id {
                    memory_cost: memory.unwrap_or(19456),
                    time_cost: time.unwrap_or(2),
                    parallelism: 1,
                },
                "argon2i" => HasherConfig::Argon2i {
                    memory_cost: memory.unwrap_or(19456),
                    time_cost: time.unwrap_or(2),
                    parallelism: 1,
                },
                "argon2d" => HasherConfig::Argon2d {
                    memory_cost: memory.unwrap_or(19456),
                    time_cost: time.unwrap_or(2),
                    parallelism: 1,
                },
                "bcrypt" => HasherConfig::Bcrypt {
                    cost: work_factor.unwrap_or(12),
                },
                "scrypt" => HasherConfig::Scrypt {
                    n: work_factor.unwrap_or(16384),
                    r: 8,
                    p: 1,
                },
                "pbkdf2" => HasherConfig::Pbkdf2 {
                    iterations: iterations.unwrap_or(100000),
                },
                "sha256" => HasherConfig::Sha256,
                "sha512" => HasherConfig::Sha512,
                "md5" => HasherConfig::Md5,
                _ => return Err(SaltError::InvalidAlgorithm(algorithm)),
            };

            // Custom salt is not supported yet by Hasher::hash trait, ignoring it for now as per current trait definition
            // but the CLI arg is kept for future expansion and compatibility with project definition.
            if salt.is_some() {
                eprintln!("Warning: Custom salt is not yet supported and will be ignored. A random salt will be generated.");
            }

            let hash = hasher.hash(password.as_bytes(), &config)?;

            match output {
                OutputFormat::Plain => {
                    println!("{}", hash);
                }
                OutputFormat::Json => {
                    let res = HashResult::from_hash(&hash);
                    println!("{}", serde_json::to_string_pretty(&res)?);
                }
            }
        }
        Commands::Verify {
            password: cli_password,
            hash: cli_hash,
            output,
            verbose,
        } => {
            let (password, hash) = match (cli_password, cli_hash) {
                (Some(p), Some(h)) => (get_password(Some(p))?, h),
                (Some(h), None) => (get_password(None)?, h),
                (None, _) => {
                    return Err(SaltError::InvalidParameter("Hash is required for verification".to_string()));
                }
            };

            let mut success = false;

            for hasher in registry.all_hashers() {
                if verbose {
                    eprintln!("Trying {}...", hasher.name());
                }
                match hasher.verify(password.as_bytes(), &hash) {
                    Ok(true) => {
                        success = true;
                        break;
                    }
                    _ => continue,
                }
            }

            match output {
                OutputFormat::Plain => {
                    if success {
                        let res = HashResult::from_hash(&hash);
                        println!("✓ Password matches");
                        println!("Algorithm: {}", res.algorithm);
                    } else {
                        println!("✗ Password does not match");
                    }
                }
                OutputFormat::Json => {
                    if success {
                        let res = HashResult::from_hash(&hash);
                        println!("{}", serde_json::to_string_pretty(&res)?);
                    } else {
                        let mut map = HashMap::new();
                        map.insert("matches", false);
                        println!("{}", serde_json::to_string(&map)?);
                    }
                }
            }
            if !success {
                std::process::exit(1);
            }
        }
        Commands::Algorithms { output } => {
            let mut algos = registry.list_algorithms();
            algos.sort();
            match output {
                OutputFormat::Plain => {
                    println!("Available algorithms:");
                    for algo in algos {
                        println!("  - {}", algo);
                    }
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string(&algos)?);
                }
            }
        }
    }

    Ok(())
}
