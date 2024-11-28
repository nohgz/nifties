use std::io::{self, Write};
use std::collections::HashMap;

// -----------------------------------------------------------------------------
// Nifty Hawaiian Phonetic Generator
//
// Description:
// Rust implementation of Stanford Nifty Assignment "Hawaiian Phonetic Generator"
// http://nifty.stanford.edu/2019/bingham-hawaiian-phonetic-generator/
//
// Usage:
// Run with cargo, no args necessary
//   cargo run
//
// Dependencies: none
// -----------------------------------------------------------------------------

// Build the vowel hash map
fn build_vowels() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("a", "ah");
    map.insert("e", "eh");
    map.insert("i", "ee");
    map.insert("o", "oh");
    map.insert("u", "oo");
    map
}

// Build the vowel pairs hash map
fn build_vowel_pairs() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("ai", "eye");
    map.insert("ae", "eye");
    map.insert("ao", "ow");
    map.insert("au", "ow");
    map.insert("ei", "ay");
    map.insert("eu", "eh-oo");
    map.insert("iu", "ew");
    map.insert("oi", "oy");
    map.insert("ou", "ew");
    map.insert("ui", "ooey");
    map
}
fn main() -> io::Result<()> {
    let vowels = build_vowels();
    let vowel_pairs = build_vowel_pairs();

    let mut word = String::new();

    loop {
        // Input cycle 1 (input word)
        print!("Input a Hawaiian Word Here ==> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut word)?;

        // don't leave this out it will cause stupid pronunciations
        let trimmed_word = word.trim();

        if validize_word(trimmed_word) {
            println!(
                "{} is pronounced {}",
                trimmed_word,
                pronounce(trimmed_word, &vowels, &vowel_pairs)
            );
        }

        // Input cycle 2 (replay?)
        print!("Would you like to enter another? (Y/N) : ");
        io::stdout().flush()?;
        word.clear();
        io::stdin().read_line(&mut word)?;

        // Normalize the answer, trim it, and make it uppercase for consistency
        let trimmed_answer = word.trim().to_uppercase();

        // Exit the loop if the user enters "N" or "NO"
        if trimmed_answer == "N" || trimmed_answer == "NO" {
            println!("Mahalo no ka pā'ani 'ana!");
            break;
        }

        // Clear the word to use it for the next loop iteration
        word.clear();
    }

    Ok(())
}

fn validize_word(word: &str) -> bool {
    let sanitized_word = word.trim().to_lowercase();
    // Check if all characters are valid Hawaiian characters
    let is_valid = sanitized_word.chars().all(|c| " aehiklmnopuw'".contains(c));

    if !is_valid {
        // Find the first invalid character
        if let Some(invalid_char) = sanitized_word.chars().find(|&c| !" aehiklmnopuw'".contains(c)) {
            eprintln!("{} is not a valid Hawaiian Character!", invalid_char);
        }
        return false;
    }

    // otherwise, we good
    true
}

// Capitalization helper method
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Implementation based off of
// https://stackoverflow.com/questions/55291856/hawaiian-pronouncer
fn pronounce(word: &str, vowels: &HashMap<&str, &str>, vowel_pairs: &HashMap<&str, &str>) -> String {
    let chars = word.to_lowercase();
    let mut i = 0;
    let mut result = String::new();

    while i < chars.len() {
        // Extract one character
        let char = chars[i..i + 1].to_string();
        let mut tr = None;

        if i < chars.len() - 1 {
            let pair = &chars[i..i + 2];
            tr = vowel_pairs.get(pair);
        }

        // If no vowel pair found, check single character vowel
        if tr.is_none() {
            tr = vowels.get(&char as &str);
        } else {
            i += 1;
        }

        // If a translation was found, append it with a trailing dash for pairs
        if let Some(t) = tr {
            result.push_str(t);
            if i < chars.len() - 1 {
                result.push('-');
            }
        } else {
            result.push_str(&char);
        }

        // Move to next character
        i += 1;
    }

    capitalize(&result)
}