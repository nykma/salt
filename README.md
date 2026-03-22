# salt

A high-performance CLI tool for password hashing generation and verification.

`salt` provides a simple and secure interface for working with modern password hashing algorithms like Argon2, bcrypt, and scrypt.

## Features

- **Modern Algorithms**: Supports Argon2id (default), Argon2i, Argon2d, bcrypt, scrypt, and PBKDF2.
- **Legacy Support**: Supports SHA-256, SHA-512, and MD5 (with warnings).
- **Secure by Default**: Uses cryptographically secure random salts and recommended parameters.
- **Flexible Output**: Supports plain text and JSON output formats.
- **Easy Verification**: Automatically detects the algorithm used in a hash string for easy verification.
- **Command Aliases**: Use `g` for `generate`, `v` for `verify`, and `a` for `algorithms` for faster typing.

## Installation

### Binary Packages (Recommended)

Download the appropriate package for your system from the [GitHub Releases](https://github.com/nykma/salt/releases) page.

#### Debian / Ubuntu / Kali (DEB)
```bash
sudo apt install ./salt_0.1.0_amd64.deb
```

#### Fedora / RHEL / CentOS (RPM)
```bash
sudo dnf install ./salt-0.1.0-1.x86_64.rpm
```

#### Arch Linux
```bash
sudo pacman -U salt-0.1.0-1-x86_64.pkg.tar.zst
```

### Nix (Flake)

If you have Nix enabled with Flakes, you can install `salt` directly:

```bash
# Install to your profile
nix profile install github:nykma/salt

# Or run it directly without installing
nix run github:nykma/salt -- --help
```

### From Source

You can also build and install from source using Cargo:

```bash
cargo install --path .
```

## Quick Start

### Generate a hash (Argon2id)
```bash
# Securely prompt for password
salt g

# Or via stdin
echo "my-secret-password" | salt g
```

### Verify a password
```bash
# Securely prompt for password
salt v "$argon2id$v=19$m=19456,t=2,p=1$..."

# Or via stdin
echo "my-secret-password" | salt v "$argon2id$v=19$m=19456,t=2,p=1$..."
```

### Use a specific algorithm
```bash
salt g --algorithm bcrypt
```

### JSON output
```bash
salt g --output json
```

## Documentation

- [Usage Guide](docs/usage.md)
- [Supported Algorithms](docs/algorithms.md)
- [Security Best Practices](docs/security.md)

## License

MIT
