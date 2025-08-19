use std::str::FromStr;

/// Gets a string
pub fn get_string(message: &str) -> String {
    println!("{message}");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

/// Asks for an input using the provided message and will keep asking
/// until the input can successfully be parsed into the type this function gets casted to.
pub fn get_input<T: FromStr>(message: &str) -> T {
    loop {
        let input = get_string(message);
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

/// Gets a char. Will try again until user inputs a valid char.
pub fn get_char(message: &str) -> char {
    get_input(message)
}

/// Gets an integer (i32). Will try again until user inputs a valid integer.
pub fn get_i32(message: &str) -> i32 {
    get_input(message)
}

/// Gets an integer (i64). Will try again until user inputs a valid integer.
pub fn get_i64(message: &str) -> i64 {
    get_input(message)
}

/// Gets an float (f32). Will try again until user inputs a valid float.
pub fn get_f32(message: &str) -> f32 {
    get_input(message)
}

/// Gets an float (f64). Will try again until user inputs a valid float.
pub fn get_f64(message: &str) -> f64 {
    get_input(message)
}

/// Gets an unsigned integer (u32). Will try again until user inputs a valid integer.
/// If a negative integer is inputted it returns the absolute value of the number.
pub fn get_u32(message: &str) -> u32 {
    get_i32(message).abs() as u32
}
