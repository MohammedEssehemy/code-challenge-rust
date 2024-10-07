use clap::Parser;
use std::io::Write;
use utils::{read_file, println_or_exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source to read from. If empty, stdin or - it will read from stdin
    #[arg(value_name = "FILE", default_value = "stdin")]
    file: String,

    /// field indices to extract, one based. Supports multiple delimiters: comma, colon, and semicolon
    #[arg(short = 'f', long = "field", value_parser = parse_field_indices)]
    field_indices: std::vec::Vec<usize>, // fully qualified to avoid https://github.com/clap-rs/clap/issues/4679#issuecomment-1419303347

    /// Delimiter to use between fields. Defaults to tab
    #[arg(short, long, default_value = "\t")]
    delimiter: String,
}

fn parse_field_indices(input: &str) -> Result<Vec<usize>, String> {
    input
        .split(&[',', ' ',  '\t'][..]) // Updated to split by comma and space
        .filter(|s| !s.is_empty()) // Filter out empty strings
        .map(|field| {
            field
                .trim()
                .parse::<usize>()
                .map_err(|_| format!("Invalid field: {}", field))
                .map(|num| num.checked_sub(1).ok_or("Field index must be greater than 0".to_string()))
                .and_then(|result| result)
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    let file_path = args.file.unwrap_or("stdin".to_string());
    let buffer = read_file(&file_path);
    buffer.lines().for_each(|line| {
        let all_fields = line.split(&args.delimiter).collect::<Vec<_>>();
        let fields = args
            .field_indices
            .iter()
            .map(|field| all_fields.get(*field).unwrap_or(&"").to_string())
            .collect::<Vec<_>>();
        println_or_exit!("{}", fields.join("\t"));
    });
}
