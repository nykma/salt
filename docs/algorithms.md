# Supported Hashing Algorithms

`salt` supports a variety of modern and legacy hashing algorithms.

## Modern Key Derivation Functions (KDFs)

These algorithms are designed to be slow and resource-intensive to defend against brute-force attacks.

### Argon2

Argon2 is the winner of the Password Hashing Competition (2015). It has three variants:

- **Argon2id**: (Recommended) A hybrid that provides a good balance between security and performance.
- **Argon2i**: Optimized for resistance against side-channel attacks.
- **Argon2d**: Optimized for resistance against GPU cracking.

**Parameters:**
- `memory`: Memory cost (KB). Default: 19456 (19 MB).
- `time`: Time cost (iterations). Default: 2.
- `parallelism`: Degree of parallelism. Default: 1.

### bcrypt

A classic password hashing function based on the Blowfish cipher. It is a memory-efficient algorithm but very effective against CPU and GPU-based attacks.

**Parameters:**
- `work_factor`: The cost parameter (log2 of iterations). Default: 12.

### scrypt

Designed to be memory-hard to resist custom hardware (ASIC/FPGA) attacks.

**Parameters:**
- `n` (via `--work-factor`): CPU/memory cost parameter. Must be a power of 2 and greater than 1. Default: 16384.
- `r`: Block size parameter. Default: 8.
- `p`: Parallelization parameter. Default: 1.

### PBKDF2

Password-Based Key Derivation Function 2. A standard used in many protocols.

**Parameters:**
- `iterations`: Number of iterations. Default: 100000.

## Legacy and General Purpose Hashes

These algorithms are not recommended for storing passwords but are supported for compatibility and general use.

### SHA-256 / SHA-512

Part of the Secure Hash Algorithm 2 (SHA-2) family. They are fast and not designed for password hashing.

### MD5

A widely used but cryptographically broken message-digest algorithm. Provided only for legacy purposes. A warning will be displayed when using MD5.
