use std::{collections::HashSet, fs};
// use std::time::Instant;

use anyhow::{Result as AnyhowResult, bail};
const MAX_WORD_SIZE: usize = 45;

fn main() -> AnyhowResult<()> {
    // let start_time = Instant::now();
    let args: Vec<String> = {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);
        args
    };
    let dictionary_path;
    let text_path;
    if args.len() == 1 {
        dictionary_path = "dictionaries/large".to_string(); // Default dictionary.
        text_path = args[0].clone();
    } else if args.len() == 2 {
        dictionary_path = args[0].clone();
        text_path = args[1].clone();
    } else {
        bail!("Invalid input\n\nUsage: speller dictionary text.txt\n");
    }

    // Load and separate into words.
    let dictionary: HashSet<String> = get_words(&fs::read_to_string(dictionary_path)?)
        .into_iter()
        .collect();
    let text = get_words(&fs::read_to_string(text_path)?);

    // Find misspelled words.
    println!("MISSPELLED WORDS\n");
    let mut misspelled = 0;
    for word in &text {
        if !dictionary.contains(&word.to_lowercase()) {
            println!("{word}");
            misspelled += 1;
        }
    }
    // let elapsed_time = start_time.elapsed();
    println!("");
    println!("WORDS MISSPELLED:     {}", misspelled);
    println!("WORDS IN DICTIONARY:  {}", dictionary.len());
    println!("WORDS IN TEXT:        {}", text.len());
    // println!("TIME IN TOTAL: {:?}", elapsed_time);

    Ok(())
}

fn get_words(text: &String) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = Vec::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() || (word.len() != 0 && c == '\'') {
            word.push(c);
        } else {
            let len = word.len();
            if len > 0 && len <= MAX_WORD_SIZE {
                words.push(word.iter().collect());
                word.clear();
            }
        }
    }
    let len = word.len();
    if len > 0 && len <= MAX_WORD_SIZE {
        words.push(word.iter().collect());
    }

    words
}
