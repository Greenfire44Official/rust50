use std::env::args;

use anyhow::{Result, bail};
use my_library::get_string;

fn main() -> Result<()> {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        bail!("Invalid input\n\nUsage: {} key\n", args[0]);
    }
    let key: u128 = match args[1].parse() {
        Ok(k) => k,
        Err(_) => bail!("Key should be a number"),
    };
    let key: u8 = (key % 26) as u8;

    let output: String = get_string("Text to encrypt: ")
        .chars()
        .filter_map(|c| caesar_encrypt(key, c))
        .collect();
    println!("ciphertext: {output}");
    Ok(())
}

fn caesar_encrypt(key: u8, c: char) -> Option<char> {
    match c {
        c if c.is_alphabetic() => {
            let a: u8 = match c.is_lowercase() {
                true => 'a' as u8,
                false => 'A' as u8,
            };
            let c = c as u8;
            Some(((c - a + key) % 26 + a) as char)
        }
        _ => Some(c),
    }
}
