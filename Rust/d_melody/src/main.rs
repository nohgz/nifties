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

    let notes = build_note_buf(&create_file_name("travelers"), false, 1.0, -2);
    let _ = play_notes(&stream, notes.expect("REASON"));

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

fn play_notes(stream: &Sink, note_buf: Vec<Note>) -> io::Result<()> {
    let mut replay_buf: Vec<Note> = Vec::new();
    let mut replay_mode = false;

    for note in note_buf {
        play_note(&stream, note.clone());

        /*
         * Stupid ass replay hack
         */

        // If the note is marked with repeat, toggle replay mode and push that note
        if note.repeat {
            replay_mode = !replay_mode;
            replay_buf.push(note.clone());

            // When replay mode toggles off, play buffered notes and clear the buffer
            if !replay_mode {
                for replay_note in &replay_buf {
                    play_note(&stream, replay_note.clone());
                }
                replay_buf.clear();
            }
        }

        // Added the second condition so that head and tail notes don't count twice.
        if replay_mode && !note.repeat {
            // When in replay mode, push the note into a buffer for later playing
            replay_buf.push(note.clone());
        }
    }

    Ok(())
}

fn build_note_buf(filepath: &str, reverse: bool, tempo_mod: f32, octave_mod: i32) -> io::Result<Vec<Note>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut octave_max = 1;
    let mut octave_min = 10;
    let mut note_buf: Vec<Note> = Vec::new();

    // Create Notes Array
    for line in reader.lines() {
        let mut note = note_from_str(&line?);

        // Get the highest and lowest octaves
        if note.octave > octave_max {
            octave_max = note.octave;
        }
        if note.octave < octave_min {
            octave_min = note.octave;
        }

        note_buf.push(note);
    }

    // Modify the octaves and tempos
    for note in note_buf.iter_mut() {
        // Modify tempo
        note.duration /= tempo_mod;

        // Handle positive octave shift
        if octave_max + octave_mod < 10 && octave_mod > 0 {
            note.octave += octave_mod;
        } else if octave_mod > 0 {
            // if octave mod is too big, then increase as much as possible
            note.octave += 9 - octave_max;
        }

        // Handle negative octave shift
        if octave_min + octave_mod > 1 && octave_mod < 0 {
            note.octave += octave_mod;
         } else if octave_mod < 0 {
            // if octave mod is too small, then decrease as much as possible
            note.octave += 1 - octave_min;
        }
    }

    // If user asks for it, reverse the order
    if reverse {
        note_buf.reverse();
    }

    Ok(note_buf)
}


fn print_note(note: Note) {
    println!("DURATION: {} , PITCH: {}, OCTAVE: {}, ACCIDENTAL: {}, REPEAT: {}",
            note.duration, note.pitch, note.octave, note.accidental, note.repeat);
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
        _   => -69420 // invalid pitch.
    }
}

fn accidental_to_adjustment(accidental: char) -> i32 {
    match accidental {
        'S' => 1,  // Sharp
        'F' => -1, // Flat
        _ => 0,    // Natural
    }
}

fn play_note(stream: &Sink, note: Note) {
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