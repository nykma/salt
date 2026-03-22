# Usage Guide

`salt` is a high-performance CLI tool for password hashing generation and verification.

## Basic Usage

### Generate a password hash

To generate a hash for a password using the default algorithm (`argon2id`):

```bash
# Securely prompt for password
salt generate

# Or via stdin
echo "mypassword" | salt generate
```

To specify a different algorithm:

```bash
salt generate --algorithm bcrypt
```

### Verify a password

To verify a password against a hash:

```bash
# Securely prompt for password
salt verify "$argon2id$v=19$m=19456,t=2,p=1$..."

# Or via stdin
echo "mypassword" | salt verify "$argon2id$v=19$m=19456,t=2,p=1$..."
```

The tool will automatically detect the algorithm from the hash string.

### List supported algorithms

```bash
salt algorithms
```

## Argon2id Default

By default, `salt` uses **Argon2id** for hashing. Argon2id is the winner of the Password Hashing Competition (PHC) and is recommended for most use cases because it provides a good balance between resistance against side-channel attacks and GPU-based cracking.

Default parameters for Argon2id in `salt`:
- Memory: 19456 KB (19 MB)
- Time: 2 iterations
- Parallelism: 1

## Advanced Configuration

You can customize the hashing parameters:

- `--iterations`: Used by PBKDF2.
- `--work-factor`: Used by bcrypt (cost) and scrypt (n).
- `--memory`: Used by Argon2.
- `--time`: Used by Argon2.

Example for bcrypt with custom work factor:
```bash
salt generate --algorithm bcrypt --work-factor 14
```

Example for scrypt with custom n (must be a power of 2):
```bash
salt generate --algorithm scrypt --work-factor 16384
```

## Output Formats

`salt` supports `plain` (default) and `json` output formats.

```bash
salt generate --output json
```

This will provide a structured JSON output containing the hash, algorithm, and parameters used.
