use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::io::Write;

// -----------------------------------------------------------------------------
// Nifty Movie Review Sentiment Analysis
//
// Description:
// Rust implementation of Stanford Nifty Assignment "Movie Review Sentiment Analysis"
// http://nifty.stanford.edu/2016/manley-urness-movie-review-sentiment/
//
// Uses a hashmap to score and index word sentiments for speeeed
//
// Usage:
// Run with cargo, no args necessary
//   cargo run
//
// Dependencies: none
// -----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    // create hashmap for indexing words. its super duper fast :o
    let mut word_map: HashMap<String, (usize, f64)> = HashMap::new();
    let _ = populate_hashmap(&mut word_map, "assets/movieReviews.txt");

    loop {
        println!("\nWhat would you like to do?");
        println!("1: Get the score of a word");
        println!("2: Get the average score of words in a file (one word per line)");
        println!("3: Find the highest/lowest scoring words in a file");
        println!("4: Sort words from a file into positivewords.txt and negativewords.txt");
        println!("5: Exit the program");

        println!("Enter your choice (1-5): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            // simple find word
            "1" => {
                println!("Enter a word:");
                let mut word = String::new();
                io::stdin().read_line(&mut word)?;
                let word = word.trim();
                if let Some((_, avg_score)) = find_word(&word_map, word) {
                    println!(
                        "The word '{}' appears in the reviews with an average score of {:.2}.",
                        word, avg_score
                    );
                } else {
                    println!("The word '{}' was not found in the reviews.", word);
                }
            }
            // average score of words in file
            "2" => {
                println!("Enter the path to the file:");
                let mut path = String::new();
                io::stdin().read_line(&mut path)?;
                let path = path.trim();
                file_average_sentiment(&word_map, path)?;
            }
            // max / min score of words in file
            "3" => {
                println!("Enter the path to the file:");
                let mut path = String::new();
                io::stdin().read_line(&mut path)?;
                let path = path.trim();
                file_max_min_sentiment(&word_map, path)?;
            }
            // sort into positive / negative words from file
            "4" => {
                println!("Enter the path to the file:");
                let mut path = String::new();
                io::stdin().read_line(&mut path)?;
                let path = path.trim();
                file_print_sentiment(&word_map, path)?;
            }
            // exit
            "5" => {
                println!("Goodbye!");
                break;
            }
            // invalid
            _ => {
                println!("Only enter a number between 1-5.");
            }
        }
    }

    Ok(())
}

// populates the map of total scores and word occurrences
fn populate_hashmap(word_map: &mut HashMap<String, (usize, f64)>, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // split each line into the score and review part
        let mut parts = line.splitn(2, ' ');
        if let (Some(score_str), Some(review)) = (parts.next(), parts.next()) {
            let score: f64 = score_str.parse().unwrap();
            for word in review.split_whitespace() {
                // inserts a unique word and adds the review score to a total
                // so its sentiment can be calculated later when needed
                // this way saves CPU cycles so it doesn't calculate an average
                // score of a word no one cares about
                let entry = word_map.entry(word.to_string()).or_insert((0, 0.0));
                entry.0 += 1;
                entry.1 += score;
            }
        }
    }
    Ok(())
}

// finds a word in the hashmap and returns its average score
fn find_word(word_map: &HashMap<String, (usize, f64)>, word: &str) -> Option<(usize, f64)> {
    word_map.get(word).map(|&(count, total_score)| (count, total_score / count as f64))
}

// reports average sentiment of a file
fn file_average_sentiment(word_map: &HashMap<String, (usize, f64)>, words_path: &str) -> io::Result<()> {
    let file = File::open(words_path)?;
    let reader = io::BufReader::new(file);
    let mut total_avg_score: f64 = 0.0;
    let mut word_count: usize = 0;

    for line in reader.lines() {
        let word = line?;
        if let Some((_, avg_score)) = find_word(word_map, &word) {
            total_avg_score += avg_score;
            word_count += 1;
        } else {
            println!("Found no occurrences of '{}'.", word);
        }
    }

    if word_count > 0 {
        let overall_avg_score = total_avg_score / word_count as f64;
        let sentiment = if overall_avg_score >= 2.0 {
            "positive"
        } else {
            "negative"
        };
        println!(
            "The overall sentiment of '{}' is {}, with an average score of {:.2}.",
            words_path, sentiment, overall_avg_score
        );
    } else {
        println!("An overall sentiment cannot be estimated for this file.");
    }

    Ok(())
}

// reports the most and least positive sentiments for a file
fn file_max_min_sentiment(word_map: &HashMap<String, (usize, f64)>, words_path: &str) -> io::Result<()> {
    let file = File::open(words_path)?;
    let reader = io::BufReader::new(file);

    let mut max_score = f64::MIN;
    let mut min_score = f64::MAX;
    let mut max_word = String::new();
    let mut min_word = String::new();

    for line in reader.lines() {
        let word = line?;
        if let Some((_, avg_score)) = find_word(word_map, &word) {
            // simple logic to get a max and min score
            if avg_score > max_score {
                max_score = avg_score;
                max_word = word.clone();
            }
            if avg_score < min_score {
                min_score = avg_score;
                min_word = word.clone();
            }
        }
    }

    // if was able to find a maximum and minimum word, report them
    if max_score != f64::MIN && min_score != f64::MAX {
        println!("The most positive word is '{}' with a score of {:.2}", max_word, max_score);
        println!("The most negative word is '{}' with a score of {:.2}", min_word, min_score);
    } else {
        println!("No words in the file matched any entries in the word map.");
    }

    Ok(())
}

// prints sentiment of words into positive and negative files
fn file_print_sentiment(word_map: &HashMap<String, (usize, f64)>, words_path: &str) -> io::Result<()> {
    let file = File::open(words_path)?;
    let reader = io::BufReader::new(file);

    let mut positive_words = Vec::new();
    let mut negative_words = Vec::new();

    for line in reader.lines() {
        let word = line?;
        if let Some((_, avg_score)) = find_word(word_map, &word) {
            if avg_score > 2.1 {
                positive_words.push(word.clone());
            } else if avg_score < 1.9 {
                negative_words.push(word.clone());
            }
        }
    }

    let mut positive_file = File::create("positivewords.txt")?;
    for word in positive_words {
        writeln!(positive_file, "{}", word)?;
    }

    let mut negative_file = File::create("negativewords.txt")?;
    for word in negative_words {
        writeln!(negative_file, "{}", word)?;
    }

    println!("Positive words saved to 'positivewords.txt', negative words saved to 'negativewords.txt'.");

    Ok(())
}