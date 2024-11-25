use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use std::mem;
use rand::Rng;
use std::env;

fn main() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut filename;
    let mut file;

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    /*
        FOR LOOP FOR MINUETS
    */
    for i in 0..16 {
        filename = create_file_name(&args[1], "minuet", i);
        file = BufReader::new(File::open(filename).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }

    /*
        FOR LOOP FOR TRIOS
    */
    for i in 0..16 {
        filename = create_file_name(&args[1], "trio", i);
        file = BufReader::new(File::open(filename).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }

    sink.sleep_until_end();
}

// Takes in instrument and part and returns file name for playing, does randomness
fn create_file_name(instrument: &str, part: &str, phrase: i32) -> String {
    let num = if part == "minuet"{ rand::thread_rng().gen_range(2..12) } else { rand::thread_rng().gen_range(1..6) };
    return format!("assets/{}/{}{}-{}.wav", instrument, part, phrase, num);
}