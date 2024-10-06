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
    let file_path = args.file.unwrap_or("stdin".to_string());
    let file_content = if file_path == "stdin" {
        read_file_from_stdin()
    } else {
        read_file_from_path(&file_path)
    };

    let mut output = Vec::new();
    let default_mode = !args.count && !args.lines && !args.words && !args.multi;

    if args.lines || default_mode {
        output.push(file_content.lines().count().to_string());
    }
    if args.words || default_mode {
        output.push(file_content.split_whitespace().count().to_string());
    }
    if args.count || default_mode {
        output.push(file_content.bytes().count().to_string());
    }
    if args.multi {
        output.push(file_content.chars().count().to_string());
    }

    output.push(file_path);
    println!("{}", output.join(" "));
}
