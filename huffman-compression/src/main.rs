use huffman_compression::HuffmanCompression;
use clap::Parser;
use utils::read_file;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct HuffmanCommand {
    /// Source to read from. If empty, stdin or - it will read from stdin
    #[arg(value_name = "INPUT_FILE", default_value = "stdin")]
    input_file: String,

    /// Output file to write to. If empty, stdout or - it will write to stdout
    #[arg(short, long = "output",value_name = "OUTPUT_FILE", default_value = "stdout")]
    output_file: String,
}

fn main() {
    let args = HuffmanCommand::parse();
    let file_path = args.input_file;
    let file_contents = read_file(&file_path);
    let huffman_compression = HuffmanCompression::encode(&file_contents);
    huffman_compression.export(&args.output_file);
}
