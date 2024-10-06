use clap::Parser;
use std::io::Write;
use utils::{read_file, println_or_exit};

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

fn main() {
    let args = Args::parse();
    let file_path = args.file.unwrap_or("stdin".to_string());
    let file_content = read_file(&file_path);

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
    println_or_exit!("{}", output.join(" "));
}
