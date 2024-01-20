use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let mut encoder = DeflateEncoder::new(output_file, Compression::default());

    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;

    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut decoder = DeflateDecoder::new(input_file);
    let mut output_file = File::create(output_path)?;

    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    output_file.write_all(&buffer)?;

    Ok(())
}

fn main() {
//    Decompress file
    let input_path = "testfile.png.crunch";
    let output_path = "testfile2.png";

    match decompress_file(input_path, output_path) {
        Ok(_) => println!("File decompressed successfully!"),
        Err(e) => println!("Error: {}", e),
    }

//    Compress file
//    let input_path = "testfile.png";
//    let output_path = "testfile.png.crunch";
//
//    match compress_file(input_path, output_path) {
//        Ok(_) => println!("File compressed successfully!"),
//        Err(e) => println!("Error: {}", e),
//    }
}