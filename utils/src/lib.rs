use std::fs::{File, write};
use std::io::{self, BufReader, Read, Write};

/// Macro for printing to stdout with error handling
///
/// This macro attempts to print to stdout and handles potential errors.
/// If a broken pipe error occurs, it exits the program gracefully.
/// For any other error, it panics with an error message.
///
/// # Usage
///
/// ```
/// println_or_exit!("Hello, {}!", "world");
/// ```
///
/// # Panics
///
/// Panics if there's an error writing to stdout that is not a broken pipe.
#[macro_export]
macro_rules! println_or_exit {
    ($($arg:tt)*) => {
        let mut handle = std::io::stdout().lock();
        if let Err(e) = writeln!(handle, $($arg)*) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
            panic!("Failed to print to stdout: {}", e);
        }
    };
}

pub fn print_or_exit<T: AsRef<[u8]>>(arg: T) {
    let mut handle = std::io::stdout().lock();
    if let Err(e) = handle.write_all(arg.as_ref()) {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
        panic!("Failed to print to stdout: {}", e);
    }
}

/// Read file from file path
///
/// ### Arguments
///
/// * `file_path` - File path to read, if stdin or - will read from stdin
///
/// ### Returns
///
/// * `String` - File content
pub fn read_file(file_path: &str) -> String {
    let mut buffer = String::new();

    match file_path {
        "stdin" | "-" => {
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read stdin");
        }
        _ => {
            let file = File::open(file_path).expect("Failed to open file");
            BufReader::new(file)
                .read_to_string(&mut buffer)
                .expect("Failed to read file");
        }
    }
    buffer
}


/// Read binary file from file path
///
/// ### Arguments
///
/// * `file_path` - File path to read, if stdin or - will read from stdin
///
/// ### Returns
///
/// * `Vec<u8>` - File content as bytes
pub fn read_binary_file(file_path: &str) -> Vec<u8> {
    let mut buffer = Vec::new();

    match file_path {
        "stdin" | "-" => {
            io::stdin()
                .read_to_end(&mut buffer)
                .expect("Failed to read stdin");
        }
        _ => {
            let file = File::open(file_path).expect("Failed to open file");
            BufReader::new(file)
                .read_to_end(&mut buffer)
                .expect("Failed to read file");
        }
    }
    buffer
}

/// Write contents to file path
///
/// ### Arguments
///
/// * `file_path` - File path to read, if stdout or - will write to stdin
///
/// ### Returns
///
/// * `String` - File content
pub fn write_file(file_path: &str, buffer: &Vec<u8>) {
    match file_path {
        "stdout" | "-" => { print_or_exit( buffer); }
        _ => { write(file_path, buffer).expect("Failed to write file"); }
    }
}