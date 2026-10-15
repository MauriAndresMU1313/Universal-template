use std::fs;
use std::io;
use std::path::Path;

const CODE_BASE: &str = include_str!("../templates/codeBase.R");

pub fn create_code_base(root: &Path) -> io::Result<()> {
    let file_path = root.join("src/codeBase.R");
    fs::write(&file_path, CODE_BASE)?;
    println!(" Created: {}", file_path.display());

    Ok(())
}