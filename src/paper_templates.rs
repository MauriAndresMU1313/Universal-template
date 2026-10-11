use std::io;
use std::fs;
use std::path::Path;

const PAPER_DRAFT_DIR: &str = "documents/papers_draft";

const PAPER_TEMPLATES: [(&str, &[u8]); 5] = [
    ("02a_abstract.docx", include_bytes!("../templates/02a_abstract.docx")),
    ("02b_introduction.docx", include_bytes!("../templates/02b_introduction.docx")),
    ("02c_results.docx", include_bytes!("../templates/02c_results.docx")),
    ("02d_discussion.docx", include_bytes!("../templates/02d_discussion.docx")),
    ("02e_methodology.docx", include_bytes!("../templates/02e_methodology.docx"))
    // Reference is not added because direct link must be provide it into the documents (introduction and discussion)
];

pub fn create_paper_templates(root: &Path) -> io::Result<()> {
    let target_dir = root.join(PAPER_DRAFT_DIR);
    fs::create_dir_all(&target_dir)?;

    for (file_name, content) in PAPER_TEMPLATES {
        let file_path = target_dir.join(file_name);
        fs::write(&file_path, content)?;
        println!(" Created: {}", file_path.display())
    }

    Ok(())

}