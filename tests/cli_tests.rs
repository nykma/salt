use std::process::Command;
use std::str;

#[test]
fn test_cli_generate_argon2id() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "password123"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let hash = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(hash.starts_with("$argon2id$"));
}

#[test]
fn test_cli_generate_bcrypt() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "password123", "--algorithm", "bcrypt"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let hash = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(hash.starts_with("$2b$") || hash.starts_with("$2a$") || hash.starts_with("$2y$"));
}

#[test]
fn test_cli_verify_success() {
    // First generate a hash
    let gen_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "secret-password"])
        .output()
        .expect("Failed to generate hash");
    
    let hash = str::from_utf8(&gen_output.stdout).unwrap().trim();

    // Then verify it
    let verify_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "verify", "secret-password", hash])
        .output()
        .expect("Failed to verify password");

    assert!(verify_output.status.success());
    let stdout = str::from_utf8(&verify_output.stdout).unwrap();
    assert!(stdout.contains("✓ Password matches"));
}

#[test]
fn test_cli_verify_failure() {
    // First generate a hash
    let gen_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "secret-password"])
        .output()
        .expect("Failed to generate hash");
    
    let hash = str::from_utf8(&gen_output.stdout).unwrap().trim();

    // Then verify with wrong password
    let verify_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "verify", "wrong-password", hash])
        .output()
        .expect("Failed to verify password");

    // Should fail with exit code 1
    assert!(!verify_output.status.success());
    let stdout = str::from_utf8(&verify_output.stdout).unwrap();
    assert!(stdout.contains("✗ Password does not match"));
}

#[test]
fn test_cli_verify_stdin() {
    // First generate a hash
    let gen_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "secret-password"])
        .output()
        .expect("Failed to generate hash");
    
    let hash = str::from_utf8(&gen_output.stdout).unwrap().trim();

    // Then verify it using stdin
    use std::io::Write;
    let mut child = Command::new("cargo")
        .args(&["run", "--quiet", "--", "verify", hash])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(b"secret-password").expect("Failed to write to stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("✓ Password matches"));
}

#[test]
fn test_cli_algorithms() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "algorithms"])
        .output()
        .expect("Failed to list algorithms");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("argon2id"));
    assert!(stdout.contains("bcrypt"));
    assert!(stdout.contains("scrypt"));
    assert!(stdout.contains("pbkdf2"));
}

#[test]
fn test_cli_json_output() {
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "password123", "--output", "json"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(stdout).unwrap();
    
    assert!(json["password_hash"].is_string());
    assert_eq!(json["algorithm"], "argon2id");
}

#[test]
fn test_cli_verify_json_output() {
    // First generate a hash
    let gen_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "secret-password", "-a", "argon2id"])
        .output()
        .expect("Failed to generate hash");
    
    let hash = str::from_utf8(&gen_output.stdout).unwrap().trim();

    // Then verify it with JSON output
    let verify_output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "verify", "secret-password", hash, "--output", "json"])
        .output()
        .expect("Failed to verify password");

    assert!(verify_output.status.success());
    let stdout = str::from_utf8(&verify_output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(stdout).unwrap();
    
    assert!(json["password_hash"].is_string());
    assert_eq!(json["algorithm"], "argon2id");
    assert!(json["parameters"]["memory_cost"].is_string());
    assert!(json["salt"].is_string());
}

#[test]
fn test_cli_scrypt_bug_fix() {
    // Test scrypt with n=16384 (default)
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "password123", "--algorithm", "scrypt"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let hash = str::from_utf8(&output.stdout).unwrap().trim();
    assert!(hash.starts_with("$scrypt$"));

    // Test scrypt with invalid n (not a power of 2)
    let output_invalid = Command::new("cargo")
        .args(&["run", "--quiet", "--", "generate", "password123", "--algorithm", "scrypt", "--work-factor", "1000"])
        .output()
        .expect("Failed to execute command");

    assert!(!output_invalid.status.success());
    let stderr = str::from_utf8(&output_invalid.stderr).unwrap();
    assert!(stderr.contains("scrypt 'n' must be a power of 2"));
}
