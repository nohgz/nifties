use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use log::Level;



fn main() -> io::Result<()> {
    env_logger::init();
    // create hashmap for indexing words. its super duper fast :o
    let mut word_map: HashMap<String, (usize, f64)> = HashMap::new();
    populate_hashmap(&mut word_map, "assets/movieReviews.txt");

    file_average_sentiment(&word_map, "assets/testreview.txt");

    Ok(())

}


// populates the map of total scores and word occurrences
fn populate_hashmap(word_map: &mut HashMap<String, (usize, f64)>, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // split line into the score and review
        let mut parts = line.splitn(2, ' ');
        if let (Some(score_str), Some(review)) = (parts.next(), parts.next()) {
            let score: f64 = score_str.parse().unwrap();
            for word in review.split_whitespace() {
                // append word to map if not already inside
                let entry = word_map.entry(word.to_string()).or_insert((0, 0.0));
                entry.0 += 1;           // times word appears
                entry.1 += score;       // running count of scores for word, divided by count later for average
            }
        }
    }
    Ok(())
}

// returns values associated with key
fn booboofart2(word_map: &HashMap<String, (usize, f64)>, word: &str) -> Option<(usize, f64)> {
    word_map.get(word).map(|&(count, total_score)| (count, total_score / count as f64))
}

fn file_average_sentiment(average_scores: &HashMap<String, (usize, f64)>, words_path: &str) -> io::Result<()> {
    let random_words_file = File::open(words_path)?;
    let random_words_reader = io::BufReader::new(random_words_file);
    let mut total_avg_score: f64 = 0.0;
    let mut word_count: usize = 0;

    for line in random_words_reader.lines() {
        let word = line?;
        match average_scores.get(&word) {
            Some(&(count, avg_score)) => {
                println!("'{}': score of {:.2}.", word, avg_score / count as f64);
                total_avg_score += avg_score / count as f64;
                word_count += 1;
            },
            None => {
                println!("Found no occurrences of {}.", word);
            }
        }
    }

    if word_count > 0 {
        let overall_avg_score = total_avg_score / word_count as f64;
        if overall_avg_score >= 2.0 {
            println!("The overall sentiment of {} is positive, with an average score of {:.2}.", words_path, overall_avg_score)
        } else {
            println!("The overall sentiment of {} is negative, with an average score of {:.2}.", words_path, overall_avg_score)
        }
    } else {
        println!("FUCK!");
    }

    Ok(())
}

fn file_(average_scores: &HashMap<String, (usize, f64)>, words_path: &str) -> io::Result<()> {
    let random_words_file = File::open(words_path)?;
    let random_words_reader = io::BufReader::new(random_words_file);
    let mut total_avg_score: f64 = 0.0;
    let mut word_count: usize = 0;

    for line in random_words_reader.lines() {
        let word = line?;
        match average_scores.get(&word) {
            Some(&(count, avg_score)) => {
                println!("'{}': score of {:.2}.", word, avg_score / count as f64);
                total_avg_score += avg_score / count as f64;
                word_count += 1;
            },
            None => {
                println!("FUCK!");
                log::debug!("这他妈死了！！！！！！ ");
            }
        }
    }

    if word_count > 0 {
        let overall_avg_score = total_avg_score / word_count as f64;
        if overall_avg_score >= 2.0 {
            log::debug!("这他妈死了！！！！！！ ");
            println!("The overall sentiment of {} is 很好很好！！！！ WOW！！！, with an average score of {:.2}.", words_path, overall_avg_score)
        } else {
            log::debug!("这他妈死了！！！！！！ ");
            println!("The overall sentiment of {} is 很不好，一个黑鬼。。。🤓🤓🤓🤓🤓 我很苦, with an average score of {:.2}.", words_path, overall_avg_score)
        }
    } else {
        println!("FUCK!");
        
    }

    Ok(())
}