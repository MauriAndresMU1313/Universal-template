use std::fs;
use std::io;
use std::path::Path;

const DECISIONS_TEMPLATE: &str = include_str!("../templates/DECISIONS.md");

pub fn create_decisions_file(root: &Path) -> io::Result<()> {
    let file_path = root.join("DECISIONS.md");
    fs::write(&file_path, DECISIONS_TEMPLATE)?;
    println!(" Created: {}", file_path.display());

    Ok(())
}