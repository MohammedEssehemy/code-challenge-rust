use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<String>,
    /// Number of bytes
    #[arg(short, long, default_value_t = false)]
    count: bool,
    /// Number of lines
    #[arg(short, long, default_value_t = false)]
    lines: bool,
    /// Number of words
    #[arg(short, long, default_value_t = false)]
    words: bool,
    /// Number of characters
    #[arg(short, long, default_value_t = false)]
    multi: bool,
}

fn read_file_from_path(file: &str) -> String {
    let mut file = File::open(file).expect("file not found");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("failed to convert buffer to string");
    file_content
}

fn read_file_from_stdin() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read from stdin");
    buffer
}

fn main() {
    let args = Args::parse();
    // read file
    let file_path = args.file.unwrap_or("stdin".to_string());
    let file_content = if file_path == "stdin" {
        read_file_from_stdin()
    } else {
        read_file_from_path(&file_path)
    };
    if args.count {
        // count bytes
        let bytes = file_content.bytes().count();
        println!("{} {}", bytes, &file_path);
    } else if args.lines {
        // count lines
        let lines = file_content.lines().count();
        println!("{} {}", lines, &file_path);
    } else if args.words {
        // count words
        let words = file_content.split_whitespace().count();
        println!("{} {}", words, &file_path);
    } else if args.multi {
        // count characters
        let chars = file_content.chars().count();
        println!("{} {}", chars, &file_path);
    } else {
        let bytes = file_content.bytes().count();
        let lines = file_content.lines().count();
        let words = file_content.split_whitespace().count();
        println!("{} {} {} {}", lines, words, bytes, &file_path);
    }
}
