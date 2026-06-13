mod emit;
mod parse;

use crate::emit::Format;
use std::{fs, path::PathBuf};

fn main() {
    let (path, format) = validate_input();
    let contents = fs::read_to_string(path).expect("unable to read the file");
    let changelog = parse::parse(&contents);

    emit::emit(changelog, format);
}

fn validate_input() -> (PathBuf, Format) {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: clog <file>");
        std::process::exit(1);
    });

    let path = PathBuf::from(&input);

    if !path.exists() {
        eprintln!("error: path does not exist");
        std::process::exit(1);
    }

    if path.is_dir() {
        eprintln!("error: path must be a file, not a directory");
        std::process::exit(1);
    }

    let format = std::env::args()
        .position(|a| a == "--format") // look for --format
        .and_then(|i| std::env::args().nth(i + 1)) // if we found it, get the value at the next index
        .unwrap_or_else(|| "html".to_string()); // otherwise, default to html

    let format = match format.as_str() {
        "bbcode" => Format::Bbcode,
        "html" => Format::Html,
        _ => {
            eprintln!("error: expected bbcode or html after --format");
            std::process::exit(1);
        }
    };

    (path, format)
}
