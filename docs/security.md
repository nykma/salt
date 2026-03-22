# Security Best Practices for Password Hashing

`salt` is designed to help you follow industry-standard security practices for password storage.

## General Recommendations

### Use Modern KDFs

Always prefer modern, memory-hard Key Derivation Functions (KDFs) like **Argon2id**, **bcrypt**, or **scrypt**. These are specifically designed to resist brute-force and hardware-accelerated (GPU/ASIC) attacks.

- **Argon2id** (Default) is the current industry gold standard.
- **bcrypt** is a well-tested alternative for environments where memory-hard hashing is less desirable.
- **scrypt** is an excellent choice for preventing ASIC/FPGA-based attacks.

### Avoid Legacy Hashes

Never use fast, general-purpose hashing functions like **MD5**, **SHA-1**, or **SHA-256** for password storage. These can be cracked at a rate of billions of attempts per second on modern hardware.

### Tuning Parameters

Each hashing algorithm has parameters that determine its computational cost. These should be tuned based on your server's hardware and the required security level.

- **Argon2**: Increase `memory` and `time` parameters as needed. Ensure it takes at least 100ms on your production hardware.
- **bcrypt**: Increase the `work_factor` as hardware improves. The default of 12 is generally sufficient as of 2024.
- **scrypt**: Increase the `n` parameter (must be a power of 2) to consume more memory and CPU.

### Salt Generation

A salt is a unique, random string added to the password before hashing. It prevents attackers from using precomputed tables (like rainbow tables) and ensures that two users with the same password will have different hashes.

`salt` automatically generates a secure, random salt for every hash using a cryptographically secure pseudo-random number generator (CSPRNG).

## Storage & Transmission

- **Store the entire PHC string**: Most modern algorithms output a string in the Password Hashing Competition (PHC) format (e.g., `$argon2id$v=19$m=19456,t=2,p=1$...`). This format includes the algorithm, parameters, salt, and the hash itself, allowing for easy verification.
- **Transmit passwords over TLS**: Always use HTTPS/TLS when transmitting passwords from a client to a server.
- **Never log passwords**: Ensure that raw passwords are never logged or stored in plain text anywhere in your system.
- **Zero-knowledge**: Passwords should be hashed as soon as they reach your server and then discarded from memory.
