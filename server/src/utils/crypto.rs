use argon2::password_hash::{Error, PasswordHash, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::{Rng, rng};
use std::iter;

pub fn generate_random_string(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rng();
    let one_char = || {
        let idx = rng.random_range(0..CHARSET.len());
        CHARSET[idx] as char
    };

    iter::repeat_with(one_char).take(len).collect()
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    match parsed_hash {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

pub fn sanitize_ssh_key(input: &str) -> String {
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");

    let cleaned_lines: Vec<String> = normalized
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();

    let mut cleaned_key = cleaned_lines.join("\n");

    if !cleaned_key.ends_with('\n') {
        cleaned_key.push('\n');
    }

    cleaned_key
}
