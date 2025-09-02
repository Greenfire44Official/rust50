use rust50::get_input;

const MAX_DIGITS: usize = 16;
const MIN_DIGITS: usize = 13;

/// American Express uses 15-digit numbers.
const AMEX_LENGHT: usize = 15;
/// American Express starting digits: 34, 37.
const AMEX_FIRST_DIGIT: u32 = 3;
const AMEX_SECOND_DIGITS: [u32; 2] = [4, 7];

/// MasterCard uses 16-digit numbers.
const MASTER_LENGHT: usize = 16;
/// MasterCard starting digits: 51, 52, 53, 54, 55.
const MASTER_FIRST_DIGIT: u32 = 5;
const MASTER_SECOND_DIGITS: [u32; 5] = [1, 2, 3, 4, 5];

/// Visa uses 13- and 16-digit numbers.
const VISA_LENGHTS: [usize; 2] = [13, 16];
/// Visa starting digit: 4.
const VISA_FIRST_DIGIT: u32 = 4;

fn main() {
    let card_numbers = vectorize(&get_input("Card number: "));
    check_validity(&card_numbers)
}

fn vectorize(number: &u64) -> Vec<u32> {
    number
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

fn check_validity(card_numbers: &Vec<u32>) {
    if !(MIN_DIGITS..=MAX_DIGITS).contains(&card_numbers.len()) || !luhns_checksum(&card_numbers) {
        println!("INVALID");
        return;
    }

    match find_card_processor(&card_numbers) {
        Some(vendor) => println!("{vendor}"),
        _ => println!("INVALID"),
    }
}

/// Luhn’s Algorithm
/// 1. Multiply every other digit by 2, starting with the number’s second-to-last digit, and then add those products’ digits together.
/// 2. Add the sum to the sum of the digits that weren’t multiplied by 2.
/// 3. If the total’s last digit is 0 (or, put more formally, if the total modulo 10 is congruent to 0), the number is valid!
fn luhns_checksum(card_numbers: &Vec<u32>) -> bool {
    let mut odd = 0;
    let mut even = 0;
    for (i, number) in card_numbers.iter().rev().enumerate() {
        match i % 2 {
            0 => odd += number,
            _ => even += collapse(number * 2),
        }
    }
    let total = odd + even;
    match total % 10 {
        0 => true,
        _ => false,
    }
}

fn collapse(number: u32) -> u32 {
    number % 10 + number / 10
}

fn find_card_processor(card_numbers: &Vec<u32>) -> Option<&str> {
    match card_numbers.len() {
        AMEX_LENGHT
            if card_numbers[0] == AMEX_FIRST_DIGIT
                && AMEX_SECOND_DIGITS.contains(&card_numbers[1]) =>
        {
            Some("AMEX")
        }
        MASTER_LENGHT
            if card_numbers[0] == MASTER_FIRST_DIGIT
                && MASTER_SECOND_DIGITS.contains(&card_numbers[1]) =>
        {
            Some("MASTERCARD")
        }
        x if VISA_LENGHTS.contains(&x) && card_numbers[0] == VISA_FIRST_DIGIT => Some("VISA"),
        _ => None,
    }
}
