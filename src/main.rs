use std::path::PathBuf;

use glob::glob;
use midi_msg::MidiFile;

fn main() {
    let files = find_files();

    let num_total = files.len();
    let mut num_failed_open = 0;
    let mut num_failed_cycle = 0;

    for filepath in files {
        let midi_file = match MidiFile::from_midi(&std::fs::read(&filepath).unwrap()) {
            Ok(midi_file) => midi_file,
            Err(_) => {
                num_failed_open += 1;
                println!("fail open: {filepath:?}");
                continue;
            }
        };

        match MidiFile::from_midi(&midi_file.to_midi()) {
            Ok(_) => (),
            Err(_) => {
                num_failed_cycle += 1;
                println!("fail cycle: {filepath:?}");
                continue;
            }
        }
    }

    println!("num_total:        {num_total}");
    println!(
        "num_failed_open:  {num_failed_open} ({}%)",
        num_failed_open * 100 / num_total
    );
    println!(
        "num_failed_cycle: {num_failed_cycle} ({}%)",
        num_failed_cycle * 100 / num_total
    );
}

fn find_files() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::with_capacity(23000);
    for filepath in glob("SS14-Midis/**/*.mid").unwrap() {
        paths.push(filepath.unwrap());
    }
    paths
}
