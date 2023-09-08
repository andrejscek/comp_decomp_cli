# comp_decomp_cli

**comp_decomp_cli** is a simple Rust project I made for learning purposes. It provides a Command-Line Interface (CLI) for compressing and decompressing files using the Gzip compression algorithm.

## Installation

To install this CLI tool, you'll need Rust and Cargo installed on your system. If you haven't already, you can install Rust and Cargo by following the instructions at [Rust's official website](https://www.rust-lang.org/tools/install).

You can also clone this project from its Git repository and build it manually:

```bash
git clone https://github.com/andrejscek/comp_decomp_cli.git
cd comp_decomp_cli
cargo build --release
```

## Testing

You can run the project's tests using cargo test. This will execute the test suite and provide you with the test results.

```bash
cd comp_decomp_cli
cargo test
```

## Usage

### Compression

To compress a file, use the following command:

```bash
comp_decomp_cli c source_file.txt compressed_file.gz
```

Replace `source_file.txt` with the name of the file you want to compress and `compressed_file.gz` with the desired name for the compressed output file.

### Decompression

To decompress a Gzip-compressed file, use the following command:

```bash
comp_decomp_cli d compressed_file.gz decompressed_file.txt
```

Replace `compressed_file.gz` with the name of the compressed file you want to decompress and `decompressed_file.txt` with the desired name for the decompressed output file.
