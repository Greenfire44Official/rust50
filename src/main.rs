use my_library::get_input;
use rand::Rng;
use std::{cmp::Ordering, ops::RangeInclusive};

const RANGE: RangeInclusive<i32> = 1..=100;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(RANGE.clone());
    loop {
        let guess = get_input("Please input your guess: ");

        match guess {
            x if !RANGE.contains(&x) => {
                println!("Please input a number between 1 and 100");
                continue;
            }
            _ => {}
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
