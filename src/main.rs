use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write, Seek, Cursor};
use flate2::write::GzEncoder;

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

fn create_crunch_archive(files: Vec<&str>, archive_path: &str) -> io::Result<()> {
    let mut archive_file = File::create(archive_path)?;

    for file_path in files {
        let mut file = File::open(file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        // Compress file data
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&contents)?;
        let compressed_contents = encoder.finish()?;

        // Write file header
        // File name length (4 bytes)
        let file_name = file_path.split('/').last().unwrap_or_default();
        let file_name_length = file_name.len() as u32;
        archive_file.write_all(&file_name_length.to_le_bytes())?;

        // File name (variable length)
        archive_file.write_all(file_name.as_bytes())?;

        // File size (8 bytes)
        let file_size = compressed_contents.len() as u64;
        archive_file.write_all(&file_size.to_le_bytes())?;

        // Write compressed file data
        archive_file.write_all(&compressed_contents)?;
    }

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