use clap::Parser;
use std::fs::File;
use std::{io, io::Read};

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

fn read_file(mut reader: Box<dyn Read>) -> io::Result<String> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn process_file(file: Option<&str>) -> io::Result<String> {
    match file {
        Some("stdin") | None => read_file(Box::new(io::stdin())),
        Some(path) => {
            let file = File::open(path)?;
            read_file(Box::new(file))
        }
    }
}

fn main() {
    let args = Args::parse();
    let file_content = match process_file(args.file.as_deref()) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file content: {}", err);
            return;
        }
    };
    let file_name = args.file.unwrap_or_else(|| "stdin".to_string());

    let result = match () {
        _ if args.count => file_content.bytes().count(),
        _ if args.lines => file_content.lines().count(),
        _ if args.words => file_content.split_whitespace().count(),
        _ if args.multi => file_content.chars().count(),
        _ => {
            let lines = file_content.lines().count();
            let words = file_content.split_whitespace().count();
            let bytes = file_content.bytes().count();
            println!("{} {} {} {}", lines, words, bytes, file_name);
            return;
        }
    };

    println!("{} {}", result, file_name);
}
