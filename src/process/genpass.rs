use anyhow::Result;
use rand::prelude::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"!@#$%^&*()_+-=";

pub fn parse_length(s: &str) -> std::result::Result<u8, &'static str> {
    match s.parse::<u8>() {
        Ok(n) => {
            if n < 4 {
                Err("Password length must be at least 4")
            } else {
                Ok(n)
            }
        }
        Err(_) => Err("Invalid number"),
    }
}

pub fn generate_password(
    length: u8,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    special: bool,
) -> Result<String> {
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if lowercase {
        // get one char from LOWER
        chars.extend_from_slice(LOWER);
        password.push(LOWER[rand::random::<usize>() % LOWER.len()]);
    }
    if uppercase {
        // get one char from UPPER
        chars.extend_from_slice(UPPER);
        password.push(UPPER[rand::random::<usize>() % UPPER.len()]);
    }
    if numbers {
        chars.extend_from_slice(NUMBER);
        password.push(NUMBER[rand::random::<usize>() % NUMBER.len()]);
    }
    if special {
        chars.extend_from_slice(SPECIAL);
        password.push(SPECIAL[rand::random::<usize>() % SPECIAL.len()]);
    }

    for _ in 0..length - password.len() as u8 {
        password.push(chars[rand::random::<usize>() % chars.len()]);
    }

    password.shuffle(&mut rand::thread_rng());
    let result = String::from_utf8(password)?;
    Ok(result)
}
