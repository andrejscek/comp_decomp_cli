extern crate comp_decomp_cli;
use comp_decomp_cli::compress_file;
use comp_decomp_cli::decompress_file;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compress_and_decompress() {
        // Define test file paths
        let source_file = "test_source.txt";
        let compressed_file = "test_compressed.gz";
        let decompressed_file = "test_decompressed.txt";
        let contents = "*/1234567890$#!@#$%^&*()_+{}|:<>?~`'";

        // Create a test source file
        fs::write(source_file, contents.to_owned()).expect("Failed to create test source file");

        // Compress the test source file
        compress_file(source_file, compressed_file);

        // Decompress the compressed file
        decompress_file(compressed_file, decompressed_file);

        // Read the decompressed file
        let content =
            fs::read_to_string(decompressed_file).expect("Failed to read decompressed file");

        // Assert that the content matches the original
        assert_eq!(content, contents.to_owned());

        // Clean up test files
        fs::remove_file(source_file).expect("Failed to remove test source file");
        fs::remove_file(compressed_file).expect("Failed to remove test compressed file");
        fs::remove_file(decompressed_file).expect("Failed to remove test decompressed file");
    }
}
