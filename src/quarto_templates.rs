use std::fs;
use std::io;
use std::path::Path;

const PROOF_CONCEPT_DIR: &str = "src/01_proof_concept";

const QUARTO_TEMPLATES: [(&str, &str); 3] = [
    ("00_testing_stage.qmd", include_str!("../templates/00_testing_stage.qmd")),
    ("01a_metadata_standardization.qmd", include_str!("../templates/01a_metadata_standardization.qmd")),
    ("01b_inputs_standardization.qmd", include_str!("../templates/01b_inputs_standardization.qmd")),
];

pub fn create_quarto_templates(root: &Path) -> io::Result<()> {
    let target_dir = root.join(PROOF_CONCEPT_DIR);
    fs::create_dir_all(&target_dir)?;

    for (file_name, content) in QUARTO_TEMPLATES {
        let file_path = target_dir.join(file_name);
        fs::write(&file_path, content)?;
        println!(" Created: {}", file_path.display());
    }

    Ok(())
}