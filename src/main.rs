extern crate comp_decomp_cli;
use comp_decomp_cli::compress_file;
use comp_decomp_cli::decompress_file;
use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 4 {
        eprintln!("Usage: 'program_name' 'c/d' 'source' 'target'");
        return;
    }

    let operation = &args[1];
    let source_file = &args[2];
    let target_file = &args[3];

    match operation.as_str() {
        "c" => compress_file(source_file, target_file),
        "d" => decompress_file(source_file, target_file),
        _ => eprintln!("Invalid operation. Use 'c' to compress or 'd' to decompress.'"),
    }
}
