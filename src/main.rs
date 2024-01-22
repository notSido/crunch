use flate2::read::GzDecoder;
use std::path::Path;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write, Cursor};
use flate2::write::GzEncoder;
use clap::{App, Arg, SubCommand};

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

fn extract_crunch_archive(archive_path: &str, output_dir: &str) -> io::Result<()> {
    let mut archive_file = File::open(archive_path)?;
    let output_dir_path = Path::new(output_dir);

    while let Ok(file_name_length) = read_u32(&mut archive_file) {
        let file_name = read_string(&mut archive_file, file_name_length as usize)?;
        let file_size = read_u64(&mut archive_file)?;

        let mut compressed_data = vec![0u8; file_size as usize];
        archive_file.read_exact(&mut compressed_data)?;

        let mut decoder = GzDecoder::new(Cursor::new(compressed_data));
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;

        let mut output_file = File::create(output_dir_path.join(file_name))?;
        output_file.write_all(&decompressed_data)?;
    }

    Ok(())
}

fn read_u32<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_u64<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut buffer = [0u8; 8];
    reader.read_exact(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}

fn read_string<R: Read>(reader: &mut R, length: usize) -> io::Result<String> {
    let mut buffer = vec![0; length];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

fn main() {
    let matches = App::new("Crunch")
        .version("1.0")
        .author("Your Name")
        .about("Compresses and extracts files using the custom Crunch format.")
        .subcommand(
            SubCommand::with_name("compress")
                .about("Compresses files into a Crunch archive.")
                .arg(Arg::with_name("FILES")
                     .help("Sets the input files to compress")
                     .required(true)
                     .multiple(true)
                     .index(1))
        )
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extracts files from a Crunch archive.")
                .arg(Arg::with_name("ARCHIVE")
                     .help("Sets the Crunch archive file to extract")
                     .required(true)
                     .index(1))
                .arg(Arg::with_name("OUTPUT_DIR")
                     .help("Sets the output directory to extract files to")
                     .required(false)
                     .index(2))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("compress", compress_matches)) => {
            let files: Vec<_> = compress_matches.values_of("FILES").unwrap().collect();
            let archive_path = "my_archive.crunch";
            match create_crunch_archive(files, archive_path) {
                Ok(()) => println!("Successfully created crunch archive."),
                Err(e) => eprintln!("Error creating archive: {}", e),
            }
        },
        Some(("extract", extract_matches)) => {
            let archive_path = extract_matches.value_of("ARCHIVE").unwrap();
            let output_dir = extract_matches.value_of("OUTPUT_DIR").unwrap_or(".");
            match extract_crunch_archive(archive_path, output_dir) {
                Ok(()) => println!("Successfully extracted crunch archive."),
                Err(e) => eprintln!("Error extracting archive: {}", e),
            }
        },
        _ => unreachable!(),
    }
}