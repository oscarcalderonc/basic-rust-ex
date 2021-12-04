use std::io;

pub fn run_palindrome_check() {
    let mut the_word = String::new();
    println!("Please enter a word to analyze:");
    io::stdin().read_line(&mut the_word)
        .expect("Enter something");

    let the_word = &the_word[..the_word.len() - 1];

    println!("Is {} a palindrome? {}", the_word, is_palindrome(&the_word));
}

fn is_palindrome(word: &str) -> bool {
    let word_letters: Vec<char> = word.chars().collect();
    for idx in 0..(word_letters.len() / 2) {
        if word_letters[idx] != word_letters[word_letters.len() - idx - 1] {
            return false;
        }
    }
    true
}
