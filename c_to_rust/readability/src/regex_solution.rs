/*
    This solution mostly works but it's slow to compile, which makes check50 timeout.
    And it also gets some texts wrong, haven't figured out why.
    I could spend time fixing it, but why do so when the simple check every char individually works.
*/
use my_library::get_string;
use regex::Regex;

fn main() {
    let text = get_string("Input text: ");

    // Regexes
    let words_re = Regex::new(r"[ \n]+").unwrap();
    let sentences_re = Regex::new(r"[.!?]+").unwrap();
    let letters_re = Regex::new(r"[ \n.!?]+").unwrap();

    let stripped_text = sentences_re.replace_all(&text, "");
    let words: Vec<&str> = words_re.split(&stripped_text).collect();

    let sentences: Vec<&str> = sentences_re.split(&text).collect();

    let sentences: Vec<&str> = sentences
        .iter()
        .filter_map(|&s| match s.len() {
            0 => None,
            _ => Some(s),
        })
        .collect();

    // let letters = text
    //     .chars()
    //     .filter(|c| c.is_alphabetic())
    //     .collect::<Vec<char>>();

    let stripped_text = letters_re.replace_all(&text, "");
    let letters: Vec<char> = stripped_text.chars().collect();

    // println!(
    //     "Words: {}\nSentences: {}\nLetters: {}",
    //     words.len(),
    //     sentences.len(),
    //     letters.len()
    // );

    // println!("Sentences:");
    // for sentence in &sentences {
    //     println!("{sentence}")
    // }

    // println!("Words:");
    // for word in &words {
    //     println!("{word}")
    // }
    let index = get_index(
        letters.len() as f64,
        words.len() as f64,
        sentences.len() as f64,
    );
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
