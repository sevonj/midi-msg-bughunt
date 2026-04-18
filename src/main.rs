use std::path::{Path, PathBuf};

use glob::glob;
use midi_msg::MidiFile;

fn main() {
    let _ = std::fs::remove_dir_all("output");

    let files = find_files();

    let num_total = files.len();
    let mut num_failed_open = 0;
    let mut num_failed_cycle = 0;
    let mut num_not_a_midi = 0;

    for filepath in files {
        let bytes = &std::fs::read(&filepath).unwrap();
        if !is_midi(bytes, &filepath) {
            num_not_a_midi += 1;
            continue;
        }
        let midi_file = match MidiFile::from_midi(bytes) {
            Ok(midi_file) => midi_file,
            Err(e) => {
                num_failed_open += 1;
                println!("fail open: {filepath:?}");
                let output_path = PathBuf::from("output/fail_open").join(filepath);
                std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
                std::fs::write(&output_path, bytes).unwrap();
                std::fs::write(&output_path.with_extension("log"), format!("{e:#X?}")).unwrap();
                continue;
            }
        };

        match MidiFile::from_midi(&midi_file.to_midi()) {
            Ok(_) => (),
            Err(e) => {
                num_failed_cycle += 1;
                println!("fail cycle: {filepath:?}");
                let output_path = PathBuf::from("output/fail_cycle").join(filepath);
                std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
                std::fs::write(&output_path, bytes).unwrap();
                std::fs::write(&output_path.with_extension("log"), format!("{e:#X?}")).unwrap();
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
    println!(
        "num_not_a_midi:   {num_not_a_midi} ({}%)",
        num_not_a_midi * 100 / num_total
    );
}

fn find_files() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::with_capacity(23000);
    for filepath in glob("SS14-Midis/**/*.mid").unwrap() {
        paths.push(filepath.unwrap());
    }
    paths
}

fn is_midi(bytes: &[u8], filepath: &Path) -> bool {
    let sig: &[u8] = &bytes[0..4];

    match sig {
        b"MThd" => {
            return true;
        }
        &[b'P', b'K', 0x3, 0x4] => println!("ZIP  {filepath:?}"),
        &[0xff, 0xd8, 0xff, 0xe1] => println!("JPEG {filepath:?}"),
        b"Rar!" => println!("RAR  {filepath:?}"),
        b"RIFF" => println!("RIFF {filepath:?}"),
        b"<!DO" => println!("HTML {filepath:?}"),
        sig => println!("UNKNOWN{sig:02x?}: {filepath:?}"),
    }
    false
}
