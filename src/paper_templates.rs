use std::io;
use std::fs;
use std::path::Path;

const PAPER_DRAFT_DIR: &str = ("/documents/papers_draft");

const PAPER_DRAFT: [(&str, &[u8]); 5] = [
    ("02a_abstract.docx", include_bytes!("../templates/02a_abstract.docx")),
    ("02b_introduction.docx", include_bytes!("../templates/02b_introduction.docx")),
    ("02c_results.docx", include_bytes!("../templates/02c_results.docx")),
    ("02d_discussion.docx", include_bytes!("..templates/02d_discussion.docx")),
    ("02e_methodology.docx", include_bytes!("../templates/02e_methodology.docx"))
    // Reference is not added because direct link must be provide it into the documents (introduction and discussion)
];