# bio-init-dir

## A universal template for Bioinformatics/Computational biology

A small Rust command-line tool that generates a predefined repository structure for bioinformatics and computational projects, **based on my typical workflow**. It also ships a second tool, `manifest`, that keeps a record of the files inside a project.

> Status: draft. Built as a learning project, so expect rough edges.

## What it does

`bio-init-dir <project-name> [--papers]` creates a new project folder with:

- a standard directory layout
- a `README.md` for the new project (with the creation date)
- a shared R code base (`src/code_base.R`)
- three Quarto templates in `src/01_proof_concept/`
- a `DECISIONS.md` template to record analysis decisions
- optionally, Word templates for a paper draft (`--papers`)

## Requirements

- Rust toolchain (the project uses `edition = "2024"`, which needs a recent Rust, 1.85 or later)
- To use the generated templates: Quarto and R (the templates use R chunks and `here`, `renv` is planned)

## Installation

From the root of this repository:

```
cargo install --path .
```

This installs two executables: `bio-init-dir` and `manifest`.

## Usage

### Create a project

```
bio-init-dir my-project
bio-init-dir my-project --papers
```

| Argument | Required | Description |
|---|---|---|
| `<project-name>` | yes | Name (or path) of the folder to create |
| `--papers` | no | Also create the Word templates for a paper draft |

The flag can go before or after the project name. Unknown options and more than one project name are rejected.

**Overwrite protection:** if the target folder already exists and is not empty (or is a file), the tool prints an error and stops before creating anything.

### Generated structure

```
my-project/
├── README.md
├── DECISIONS.md
├── inputs/
├── intermediary/
├── outputs/
├── plots/
├── src/
│   ├── code_base.R
│   └── 01_proof_concept/
│       ├── 00_testing_stage.qmd
│       ├── 01a_metadata_standardization.qmd
│       └── 01b_inputs_standardization.qmd
└── documents/
    ├── reports/
    ├── presentations/
    ├── posters/
    ├── extras/
    ├── papers/
    └── papers_draft/            (Word templates only with --papers)
        ├── 02a_abstract.docx
        ├── 02b_introduction.docx
        ├── 02c_results.docx
        ├── 02d_discussion.docx
        └── 02e_methodology.docx
```

### Track files with `manifest`

Run it from the root of a generated project:

```
manifest
manifest inputs outputs documents --output manifest.csv
```

| Argument | Default | Description |
|---|---|---|
| directories | `inputs` and `outputs` | Directories to scan, recursively |
| `--output <file>` | `manifest.csv` | Where to write the manifest |

The manifest has one row per file:

| Column | Filled by | Description |
|---|---|---|
| `path` | tool | File path |
| `decision` | you | Heading of the related entry in `DECISIONS.md` |
| `size_bytes` | tool | File size |
| `modified` | tool | Last modification date |
| `sha256` | tool | Checksum of the content |
| `comment` | you | Free note |

Behavior:

- Re-running updates size, date and checksum, and keeps your `decision` and `comment` values (rows are matched by path).
- Rows of deleted or renamed files are dropped, together with their notes.
- Hidden files (such as `.DS_Store`) are skipped.
- A changed checksum on a file with a note is the signal that the note may be outdated.

### Record decisions

`DECISIONS.md` is written only at decision points (threshold, tool, parameter, excluded samples), not as a daily diary. Each entry has: decision, reason, rejected alternative, analysis type (proof of concept, ad hoc, post hoc) and affected files. Copy the entry heading into the `decision` column of `manifest.csv`.

## Repository layout

```
bio-init-dir/
├── Cargo.toml
├── templates/                  # files embedded in the executable at compile time
│   ├── *.qmd
│   ├── *.docx
│   ├── code_base.R
│   └── DECISIONS.md
└── src/
    ├── main.rs                 # bio-init-dir
    ├── quarto_templates.rs
    ├── paper_templates.rs
    ├── code_base.rs
    ├── decisions.rs
    └── bin/
        └── manifest.rs         # manifest
```

The templates are embedded with `include_str!` and `include_bytes!`, so the executables work from any directory. After editing a template, rebuild (and reinstall) to update it.

## Known limitations

- `documents/papers_draft/` is created even without `--papers`, because it is listed in the base directories.
- `manifest` does not check `--output`: pointing it at an existing file overwrites that file.
- Large files make `manifest` slow, since every file is hashed.
- Not implemented yet: automatic `git init` and `.gitignore`, an empty `.here` file for `here::here()`, `renv` setup, and copying `custom-dark.scss` for the Quarto dark theme.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

## Author

Mauricio Morales