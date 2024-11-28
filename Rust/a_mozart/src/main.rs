use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rand::Rng;
use std::env;

// -----------------------------------------------------------------------------
// Nifty Mozart Dice Game
//
// Description:
// Rust implementation of Stanford Nifty Assignment "Mozart Musical Dice Game"
// http://nifty.stanford.edu/2023/wayne-musical-dice-game/
//
// Usage:
// Run with the instrument name as an argument. Example:
//   cargo run violin
//
// Dependencies: rodio, rand
// -----------------------------------------------------------------------------

fn main() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let stream = Sink::try_new(&stream_handle).unwrap();

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for proper argument calling because I forgot to include
    // an instrument one too many times and Rust screamed at me
    if args.len() != 2 {
        eprintln!("Usage: {} <clarinet / flute-harp / mbira / piano>", args[0]);
        return;
    }

    // Append Minuets to stream
    append_phrases(&stream, &args[1], "minuet", 16);

    // Append Trios to stream
    append_phrases(&stream, &args[1], "trio", 16);

    // play the buffer of surprisingly good "music"
    stream.sleep_until_end();
}

// Takes in instrument and part and returns file name for playing, does randomness
fn create_file_name(instrument: &str, part: &str, phrase: i32) -> String {
    let mut rng = rand::thread_rng();
    // if minuet, roll 2 dice, otherwise roll 1
    let num = if part == "minuet"{ rng.gen_range(2..12) } else { rng.gen_range(1..6) };
    format!("assets/{}/{}{}-{}.wav", instrument, part, phrase, num)
}

fn append_phrases(stream: &Sink, instrument: &str, part: &str, count: i32) {
    for i in 0..count {
        // build a filename for each count and append the audio file to the stream
        let filename = create_file_name(instrument, part, i);
        let file = BufReader::new(File::open(filename).unwrap());
        let source = Decoder::new(file).unwrap();
        stream.append(source);
    }
}