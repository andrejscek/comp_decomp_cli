extern crate flate2;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

pub fn compress_file(source_file: &str, target_file: &str) {
    let mut input = BufReader::new(File::open(source_file).unwrap());
    let output = File::create(target_file).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    encoder.finish().unwrap();
    println!("Compression completed.");
    println!("Elapsed: {:?}", start.elapsed()); // end timer
}

pub fn decompress_file(source_file: &str, target_file: &str) {
    let input = BufReader::new(File::open(source_file).unwrap());
    let mut decoder = GzDecoder::new(input);

    let mut output = File::create(target_file).unwrap();
    let start = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    println!("Decompression completed.");
    println!("Elapsed: {:?}", start.elapsed()); // end timer
}
