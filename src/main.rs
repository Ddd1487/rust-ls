use std::{fs::DirEntry};
use clap::{Parser};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}
fn listing_directory(entry: &DirEntry) {
    //let dir = entry?;
    println!("{} ",entry.file_name().to_string_lossy());
}
fn reading_directory() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    for entry in std::fs::read_dir(&args.path)? {
        //let entry = entry?;
        //let path = entry.path();
        match entry {
            Ok(entry) => listing_directory(&entry),
            Err(error) => {panic!("Error listing out folders {}",error)},
        };
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    reading_directory()?;
    Ok(())
} 