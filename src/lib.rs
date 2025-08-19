/// Gets a string
pub fn get_string(message: &str) -> String {
    println!("{message}");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

/// Gets a char. Will try again until user inputs a valid char.
pub fn get_char(message: &str) -> char {
    loop {
        let input = get_string(message);
        if input.chars().count() == 1 {
            return input.chars().next().unwrap();
        }
    }
}

/// Gets an integer (i32). Will try again until user inputs a valid integer.
pub fn get_i32(message: &str) -> i32 {
    let output: i32;
    loop {
        let input = get_string(message);
        output = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    output
}

/// Gets an unsigned integer (u32). Will try again until user inputs a valid integer.
/// If a negative integer is inputted it returns the absolute value of the number.
pub fn get_u32(message: &str) -> u32 {
    get_i32(message).abs() as u32
}