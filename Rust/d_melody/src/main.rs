use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::collections::HashMap;
use std::time::Duration;

use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

// -----------------------------------------------------------------------------
// Melody
//
// Description:
// Rust implementation of Stanford Nifty Assignment "Melody"
// http://nifty.stanford.edu/2015/obourn-stepp-melody-maker/
//
// Main implements a primitive system that plays a song as requested.
// I didn't want to mess around egui.
//
// Usage:
// Run with cargo, no args necessary
//   cargo run
//
// Dependencies: rodio
// -----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    // Initialize audio stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    #[allow(unused_assignments)]
    let mut stream = Sink::try_new(&stream_handle).unwrap();

    // Create a map of available songs
    let song_map: HashMap<&str, &str> = HashMap::from([
        ("travelers", "travelers"),
        ("twinkle", "twinkle"),
        ("birthday", "birthday"),
        ("levels", "levels"),
        ("tetris", "tetris"),
        ("zombie", "zombie"),
    ]);

    let mut current_song: Option<Vec<Note>> = None;
    let mut current_song_name = String::new();
    let mut reverse = false;
    let mut tempo_mod = 1.0;
    let mut octave_mod = 0;

    loop {
        print_banner(&current_song_name, reverse, octave_mod, tempo_mod);

        print!("Enter your choice: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "A" => {
                // List available songs and let the user select one
                println!("Available songs: {}", song_map.keys().cloned().collect::<Vec<_>>().join(", "));
                print!("Enter song name: ");
                io::stdout().flush()?;

                let mut song_input = String::new();
                io::stdin().read_line(&mut song_input)?;
                let song_name = song_input.trim().to_lowercase();

                if let Some(file_key) = song_map.get(song_name.as_str()) {
                    match build_note_buf(&create_file_name(file_key)) {
                        Ok(note_buf) => {
                            current_song = Some(note_buf);
                            current_song_name = song_name.clone();
                            println!("Selected song: {}", song_name);
                        }
                        Err(e) => println!("Failed to load song: {}", e),
                    }
                } else {
                    println!("Song not found.");
                }
            }
            "S" => {
                // Play the current song
                if let Some(note_buf) = &current_song {
                    println!("Playing song: {}", current_song_name);
                    stream = Sink::try_new(&stream_handle).unwrap(); // Reset the stream
                    if let Err(e) = play_notes(&stream, note_buf.clone(), reverse, tempo_mod, octave_mod) {
                        eprintln!("Error playing song: {}", e);
                    }
                } else {
                    println!("No song selected.");
                }
            }
            "D" => {
                // Toggle reverse playback
                reverse = !reverse;
                println!("Reverse playback: {}", reverse);
            }
            "C" => {
                // Increase octave
                octave_mod += 1;
            }
            "Z" => {
                // Decrease octave
                octave_mod -= 1;
            }
            "E" => {
                // Increase tempo
                tempo_mod += 0.25;
            }
            "Q" => {
                // Decrease tempo
                tempo_mod -= 0.25;
            }
            "X" => {
                // Exit the program
                println!("Goodbye!");
                return Ok(());
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}


#[derive(Clone)]
struct Note {
    duration: f32,
    pitch: char,
    octave: i32,
    accidental: char,
    repeat: bool
}

fn print_banner(songname: &str, reverse: bool, octave_mod: i32, tempo_mod: f32) {
    // Evil print that clears the terminal
    print!("\x1B[2J\x1B[H");

    println!("---------------------------------------------------------");
    println!("-                  Rust Melody Jukebox                  -");
    println!("---------------------------------------------------------");
    println!("- NOW PLAYING: {:<40} -", songname);
    println!("- SPEED: {:>6.2}x             PITCH: {:<+4}                -", tempo_mod, octave_mod);
    println!("- REVERSE? {:<44} -", if reverse { "ENABLED" } else { "DISABLED" });
    println!("---------------------------------------------------------");
    println!("-                        Controls                       -");
    println!("---------------------------------------------------------");
    println!("- (A) Select Song   (S) Start Song   (D) Reverse Song   -");
    println!("---------------------------------------------------------");
    println!("-   (Z) Decrease Octave           (C) Increase Octave   -");
    println!("-   (Q) Decrease Tempo            (E) Increase Tempo    -");
    println!("-                        (X) Exit                       -");
    println!("---------------------------------------------------------");
}


fn create_file_name(songname: &str) -> String {
    format!("assets/songtxts/{}.txt", songname)
}

fn play_notes(stream: &Sink, note_buf: Vec<Note>, reverse: bool,
                tempo_mod: f32, octave_mod: i32,) -> io::Result<()> {
    // define sep buffer for notes that are in replay sections
    let mut replay_buf: Vec<Note> = Vec::new();
    let mut replay_mode = false;

    // Create a modifiable iterator over notes
    let mut notes = note_buf.into_iter();

    // reverse if needed
    if reverse {
        notes = notes.rev().collect::<Vec<_>>().into_iter();
    }

    for mut note in notes {
        // Apply tempo modification
        note.duration /= tempo_mod;


        // Apply octave modification
        note.octave = match note.octave.checked_add(octave_mod) {
            Some(new_octave) if new_octave >= 1 && new_octave <= 9 => new_octave,
            Some(new_octave) if new_octave > 9 => 9, // Clamp to max
            Some(new_octave) if new_octave < 1 => 1, // Clamp to min
            _ => note.octave,                       // Fallback to current octave
        };

        // Play the modified note
        play_note(&stream, note.clone());

        /*
         * Evil ass replay hack
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

        // Added the second condition so that head and tail notes don't count twice
        if replay_mode && !note.repeat {
            // When in replay mode, push the note into a buffer for later playing
            replay_buf.push(note.clone());
        }
    }
    Ok(())
}


fn build_note_buf(filepath: &str) -> io::Result<Vec<Note>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut note_buf: Vec<Note> = Vec::new();

    // Create Notes Array
    for line in reader.lines() {
        let note = note_from_str(&line?);
        note_buf.push(note);
    }

    Ok(note_buf)
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
        _   => -69420 // invalid pitch haha funny number
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