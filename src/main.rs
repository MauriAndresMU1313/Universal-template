use std::env;
use std::fs;
use std::intrinsics::simd::SimdAlign::Vector;
use std::path::Path;

fn main() -> std::io::Result<()> {
    
    // Parse project name from CLI aguments
    let arg: Vec<(String)> = env::args().collect();
    if arg.len() < 2 {
        println!("Error:Please provide a project name.");
        println!("Usage: bio-init < project name >");
        std::process::exit(1)
    }

    let project_name = &arg[1];
    let root = Path::new(project_name);
    
    // Define subdirectories
    let directories= vec![
        root.join("data/inputs"),
        root.join("data/intermediary"),
        root.join("data/outputs"),
        root.join("src"),
        root.join("documents"),
        root.join("documents/reports"),
        root.join("documents/presentations"),
        root.join("documents/extras"),
        root.join("documents/posters"),
        root.join("plots")
    ];

    println!("Initializing project structure in: {}/", project_name);

    Ok(())
}