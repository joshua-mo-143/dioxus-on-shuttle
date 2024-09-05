use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
    let thing = if std::env::var("SHUTTLE").is_ok() { true } else { false };
    // Example: Running a shell command
    let src = Path::new("public");
    let dest = if thing { Path::new(".shuttle-executables/public") } else { Path::new("target/debug/public") };


    // Copy folder recursively
    if let Err(e) = copy_dir_recursively(src, dest) {
        eprintln!("Error copying folder: {}", e);
    }

    // Emit instructions to Cargo if necessary
    println!("cargo:rerun-if-changed=build.rs");
}

fn copy_dir_recursively(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !dest.exists() {
        fs::create_dir(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursively(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
