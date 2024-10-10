use huffman_compression::HuffmanCompression;
use clap::{Parser, ValueEnum};
use utils::{read_binary_file, read_file};

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Encode,
    Decode,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct HuffmanCommand {
    /// Source to read from. If empty, stdin or - it will read from stdin
    #[arg(value_name = "INPUT_FILE", default_value = "stdin")]
    input_file: String,

    /// Output file to write to. If empty, stdout or - it will write to stdout
    #[arg(short, long = "output",value_name = "OUTPUT_FILE", default_value = "stdout")]
    output_file: String,

    /// Mode to run the program in.
    #[arg(short, long = "mode", value_name = "MODE", default_value = "encode")]
    mode: Mode,
}

fn main() {
    let args = HuffmanCommand::parse();
    let file_path = args.input_file;
    match args.mode {
        Mode::Encode => {
            let file_contents = read_file(&file_path);
            let huffman_compression = HuffmanCompression::encode(&file_contents);
            huffman_compression.export_encoded(&args.output_file);
        }
        Mode::Decode => {
            let file_contents = read_binary_file(&file_path);
            let huffman_compression = HuffmanCompression::decode(&file_contents);
            huffman_compression.export_decoded(&args.output_file);
        }
    }
}
