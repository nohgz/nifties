use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let stream = Sink::try_new(&stream_handle).unwrap();

    let _ = play_file(&stream, &create_file_name("travelers"), false);

    stream.sleep_until_end();
}

#[derive(Clone)]
struct Note {
    duration: f32,
    pitch: char,
    octave: i32,
    accidental: char,
    repeat: bool
}

fn create_file_name(songname: &str) -> String {
    format!("assets/songtxts/{}.txt", songname)
}

fn play_file(stream: &Sink, filepath: &str, reverse: bool) -> io::Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut note_buf: Vec<Note> = Vec::new();
    let mut replay_buf: Vec<Note> = Vec::new();
    let mut replay_mode = false;

    // Create Notes Array
    for line in reader.lines() {
        let note = note_from_str(&line?);
        note_buf.push(note);
    }

    // If user asks for it, reverse the order
    if reverse { note_buf.reverse(); }

    for note in note_buf {
        append_note(&stream, note.clone());

        /*
         * stupid ass hack called replay logic
         */

        // If the note is marked with repeat, toggle replay mode and push that note
        if note.repeat == true {
            replay_mode = !replay_mode;
            replay_buf.push(note.clone());

            // when replay mode toggles off, clear the buffer
            if replay_mode == false {
                for note in &replay_buf {
                    append_note(&stream, note.clone());
                }
                replay_buf.clear();
            }
        }

        // Added the second condition so that head and tail notes dont count twice.
        if replay_mode == true && note.repeat == false {
            // when in replay mode, push the note into a buffer for later playing
            replay_buf.push(note.clone());
        }
    }

    Ok(())
}

fn print_note(note: Note) {
    println!("DURATION: {} , PITCH: {}, OCTAVE: {}, ACCIDENTAL: {}, REPEAT: {}", note.duration, note.pitch, note.octave, note.accidental, note.repeat);
}

fn note_from_str(input: &str) -> Note {
    let mut strings = Vec::new();
    let mut nums = Vec::new();

    // split the string into tokens
    for token in input.split_whitespace() {
        if let Ok(num) = token.parse::<f32>() {
            nums.push(num)
        } else {
            // Otherwise, it's a string
            strings.push(token.to_string());
        }
    }

    // If strings has more than 2 entries, then its a regular note
    if strings.len() > 2 {
        Note {
            duration: nums[0] as f32,
            pitch: strings[0].chars().next().unwrap(),      // Take the first char as pitch
            octave: nums[1] as i32,
            accidental: strings[1].chars().next().unwrap(), // Take the first char as accidental
            repeat: strings[2].parse::<bool>().unwrap(),    // Parse "true" or "false" as boolean
        }
    } else {
        Note {
            duration: nums[0] as f32,
            pitch: strings[0].chars().next().unwrap(),      // Take the first char as pitch
            octave: 0,
            accidental: 'N',
            repeat: strings[1].parse::<bool>().unwrap()
        }
    }
}

fn pitch_to_semitones(pitch: char) -> i32 {
    match pitch {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _   => -69420 // invalid pitch
    }
}

fn accidental_to_adjustment(accidental: char) -> i32 {
    match accidental {
        'S' => 1,  // Sharp
        'F' => -1, // Flat
        _ => 0,    // Natural
    }
}

fn append_note(stream: &Sink, note: Note) {
    if note.pitch == 'R' {
        // Play no sound but add a delay if the note is a rest
        let source = SineWave::new(0.01).take_duration(Duration::from_secs_f32(note.duration)).amplify(0.20);
        stream.append(source);
    } else {
        let mut steps = pitch_to_semitones(note.pitch);
        // Adjust for octave
        steps += (note.octave - 4) * 12;

        // Adjust for accidental
        steps += accidental_to_adjustment(note.accidental);

        // Calculate frequency in Hz
        let hz = 440.0 * (2.0f32).powf(steps as f32 / 12.0);

        // Play the note
        let source = SineWave::new(hz).take_duration(Duration::from_secs_f32(note.duration)).amplify(0.20);
        stream.append(source);
    }
}