use std::{collections::HashSet, env::args};

use anyhow::{Result, bail};
use rust50::get_string;

fn main() -> Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        bail!("Invalid input\n\nUsage: {} key\n", args[0]);
    }
    let key = match parse_key(&args[1]) {
        Ok(k) => k,
        Err(e) => bail!(e),
    };

    let output = match encrypt(key, &get_string("Text to encrypt: ")) {
        Ok(t) => t,
        Err(e) => bail!(e),
    };
    println!("ciphertext: {output}");

    Ok(())
}

fn encrypt(key: Vec<char>, text: &String) -> Result<String> {
    let mut encrypted: Vec<char> = Vec::new();
    for character in text.chars() {
        if !character.is_ascii_alphabetic() {
            encrypted.push(character);
            continue;
        }
        let char_upper = character.to_uppercase().collect::<Vec<char>>()[0] as u8;
        let mut encrypted_char = key[(char_upper - 'A' as u8) as usize];

        if character.is_lowercase() {
            encrypted_char = encrypted_char.to_lowercase().collect::<Vec<char>>()[0]
        }

        encrypted.push(encrypted_char)
    }
    let encrypted: String = encrypted.iter().collect();

    Ok(encrypted)
}

fn parse_key(key: &String) -> Result<Vec<char>, &'static str> {
    let mut key_hashset: HashSet<char> = HashSet::new(); // For checking for duplicates
    let mut parsed_key: Vec<char> = Vec::new();

    for char in key.chars() {
        let char: char = char.to_uppercase().collect::<Vec<char>>()[0];
        if !char.is_ascii_alphabetic() {
            return Err("Key must be english alphabetical characters.");
        }
        if !key_hashset.insert(char) {
            return Err("Key must not have repeat letters.");
        }
        parsed_key.push(char);
    }
    if key_hashset.len() != 26 {
        return Err(
            "Key must be 26 characters long and include all 26 letters of the english alphabet once",
        );
    }
    Ok(parsed_key)
}
