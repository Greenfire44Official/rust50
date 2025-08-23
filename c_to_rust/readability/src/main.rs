use my_library::get_string;

const SENTENCE_ENDERS: [char; 3] = ['.', '!', '?'];

fn main() {
    let text = get_string("Input text: ");

    let [mut letters, mut words, mut sentences] = [0; 3];
    let mut previous_char: char = ' ';
    let len = text.chars().collect::<Vec<char>>().len();

    for (index, char) in text.chars().enumerate() {
        if char == ' ' && previous_char != ' ' && index + 1 != len {
            words += 1;
        } 
        if SENTENCE_ENDERS.contains(&char) && !SENTENCE_ENDERS.contains(&previous_char) {
            sentences += 1;
        } 
        if char.is_alphabetic() {
            letters += 1;
        }
        previous_char = char;
    }
    words += 1;

    let index = get_index(letters as f64, words as f64, sentences as f64);
    match index.round() as i32 {
        x if x < 1 => println!("Before Grade 1"),
        x if x >= 16 => println!("Grade 16+"),
        x if x >= 1 && x < 16 => println!("Grade {x}"),
        _ => {}
    }
}

fn get_index(letters: f64, words: f64, sentences: f64) -> f64 {
    /*
    index = 0.0588 * L - 0.296 * S - 15.8
    where L is the average number of letters per 100 words in the text,
    and S is the average number of sentences per 100 words in the text.
    */
    let l = (letters / words) * 100.0;
    let s = (sentences / words) * 100.0;
    0.0588 * l - 0.296 * s - 15.8
}
