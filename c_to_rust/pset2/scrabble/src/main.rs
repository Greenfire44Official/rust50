use std::cmp::Ordering;

use my_library::get_string;

const SCORES: [i32; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];
const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let player1_word: String = get_word("Player 1: ");
    let player2_word: String = get_word("Player 2: ");

    match get_score(&player1_word).cmp(&get_score(&player2_word)) {
        Ordering::Greater => println!("Player 1 wins!"),
        Ordering::Equal => println!("Tie!"),
        Ordering::Less => println!("Player 2 wins!"),
    }
}

fn get_word(message: &str) -> String {
    get_string(message)
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn get_score(word: &String) -> i32 {
    let mut total = 0;
    for char in word.chars() {
        let char: char = char.to_uppercase().next().unwrap();
        let score = SCORES[ALPHABET
            .iter()
            .position(|&x| x == char)
            .expect("char should be alphabetic")];
        total += score;
    }
    total
}
