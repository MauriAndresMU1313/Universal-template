use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use sha2::{Digest, Sha256};

const DEFAULT_OUTPUT: &str = "manifest.csv";
const DEFAULT_DIRECTORIES: [&str; 2] = ["inputs", "outputs"];
const HEADER: [&str; 6] = ["path", "decision", "size_bytes", "modified", "sha256", "comment"];

/// Reads the manual columns (decision, comment) of an existing manifest, keyed by path
fn read_manual_fields(manifest_path: &Path) -> HashMap<String, (String, String)> {
    let mut manual_fields = HashMap::new();

    if let Ok(mut reader) = csv::Reader::from_path(manifest_path) {
        for record in reader.records().flatten() {
            let path = record.get(0).unwrap_or("").to_string();
            let decision = record.get(1).unwrap_or("").to_string();
            let comment = record.get(5).unwrap_or("").to_string();
            manual_fields.insert(path, (decision, comment));
        }
    }

    manual_fields
}

/// Collects every non-hidden file under `directory`, recursively
fn collect_files(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        let is_hidden = path
            .file_name()
            .map(|name| name.to_string_lossy().starts_with('.'))
            .unwrap_or(false);

        if is_hidden {
            continue;
        }

        if path.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }

    Ok(())
}

fn sha256_of_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments: manifest [--output <file>] [directory ...]
    let mut output_path = PathBuf::from(DEFAULT_OUTPUT);
    let mut directories: Vec<PathBuf> = Vec::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--output" {
            match args.next() {
                Some(value) => output_path = PathBuf::from(value),
                None => {
                    eprintln!("Error: --output needs a value.");
                    std::process::exit(1);
                }
            }
        } else {
            directories.push(PathBuf::from(arg));
        }
    }

    if directories.is_empty() {
        directories = DEFAULT_DIRECTORIES
            .iter()
            .map(|directory| PathBuf::from(*directory))
            .collect();
    }

    // Read the manual columns BEFORE overwriting the file
    let manual_fields = read_manual_fields(&output_path);

    // Collect files
    let mut files: Vec<PathBuf> = Vec::new();
    for directory in &directories {
        if directory.is_dir() {
            collect_files(directory, &mut files)?;
        } else {
            println!("Warning: '{}' is not a directory, skipped.", directory.display());
        }
    }
    files.sort();

    // Write manifest
    let mut writer = csv::Writer::from_path(&output_path)?;
    writer.write_record(HEADER)?;

    for file_path in &files {
        let metadata = fs::metadata(file_path)?;
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
        let path_text = file_path.to_string_lossy().to_string();
        let (decision, comment) = manual_fields
            .get(&path_text)
            .cloned()
            .unwrap_or_default();

        writer.write_record([
            path_text,
            decision,
            metadata.len().to_string(),
            modified.format("%Y-%m-%d %H:%M:%S").to_string(),
            sha256_of_file(file_path)?,
            comment,
        ])?;
    }

    writer.flush()?;
    println!("Manifest written to {} ({} files).", output_path.display(), files.len());

    Ok(())
}