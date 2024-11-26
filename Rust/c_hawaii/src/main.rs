use phf::phf_map;
use std::io::{self, Write};

static VOWELS: phf::Map<&'static str, &'static str> = phf_map! {
    "a" => "ah",
    "e" => "eh",
    "i" => "ee",
    "o" => "oh",
    "u" => "oo",
};

static VOWEL_PAIRS: phf::Map<&'static str, &'static str> = phf_map! {
    "ai" => "eye",
    "ae" => "eye",
    "ao" => "ow",
    "au" => "ow",
    "ei" => "ay",
    "eu" => "eh-oo",
    "iu" => "ew",
    "oi" => "oy",
    "ou" => "ew",
    "ui" => "ooey"
};


fn main() -> io::Result<()> {
    let mut word = String::new();

    loop {
        // input cycle 1 (input word)
        print!("Input a Hawaiian Word Here ==> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut word)?;

        // don't leave this out it will cause stupid pronunciations
        let trimmed_word = word.trim();

        if validize_word(trimmed_word) {
            println!("{} is pronounced {}", trimmed_word, pronounce(trimmed_word));
        }

        // input cycle 2 (replay?)
        print!("Would you like to enter another? (Y/N) : ");
        io::stdout().flush()?;
        word.clear();
        io::stdin().read_line(&mut word)?;

        // Normalize the answer, trim it, and make it uppercase for consistency
        let trimmed_answer = word.trim().to_uppercase();

        // Exit the loop if the user enters "N" or "NO"
        if trimmed_answer == "N" || trimmed_answer == "NO" {
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
    let is_valid = sanitized_word.chars().all(|c| "aehiklmnopuw'".contains(c));

    if !is_valid {
        // Find the first invalid character
        if let Some(invalid_char) = sanitized_word.chars().find(|&c| !"aehiklmnopuw'".contains(c)) {
            eprintln!("{} is not a valid Hawaiian Character!", invalid_char);
        }
        return false;
    }

    // Otherwise, we good
    true
}

// Capitalization helper method
// https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn pronounce(word: &str) -> String {
    let chars = word.to_lowercase();  // Convert word to lowercase
    let mut i = 0;
    let mut result = String::new();

    while i < chars.len() {
        let char = chars[i..i+1].to_string(); // Extract a single character
        let mut tr = None;

        if i < chars.len() - 1 {
            // Try to match vowel pairs
            let pair = &chars[i..i+2];  // Take the next character as well
            tr = VOWEL_PAIRS.get(pair);
        }

        // If no vowel pair found, check single character vowel
        if tr.is_none() {
            tr = VOWELS.get(&char);
        } else {
            i += 1; // Skip the next character since it is part of a vowel pair
        }

        // If a translation was found, append it with a trailing dash for pairs
        if let Some(t) = tr {
            result.push_str(t);
            if i < chars.len() - 1 {
                result.push('-');
            }
        } else {
            // If no translation, just add the character as is
            result.push_str(&char);
        }

        i += 1; // Move to the next character
    }

    capitalize(&result)
}