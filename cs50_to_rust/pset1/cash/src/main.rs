use rust50::get_input;

const COIN_VALUES: [f64; 4] = [25.0, 10.0, 5.0, 1.0];

fn main() {
    let mut owed: f64 = loop {
        let input: i64 = get_input("Change owed (in cents): ");
        if input <= 0 {
            println!("Please input a positive integer");
            continue;
        }
        break input as f64;
    };
    let mut coins: u32 = 0;
    for coin in COIN_VALUES {
        let div = (owed / coin) as u32;
        owed -= coin * div as f64;
        coins += div;
        if div != 0 {
            println!("Added coins: {div} ({coin}c)\n");
        }
        if owed == 0.0 {
            break;
        }
    }
    println!("Minimum coins needed: {coins}");
}
