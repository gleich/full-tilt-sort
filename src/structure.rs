use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;

/// A file in the structure
pub struct File {
    path: PathBuf,
    mod_time: SystemTime,
}

/// Get all the file paths and their modification times
pub fn get_files() -> std::io::Result<Vec<File>> {
    let mut files: Vec<File> = Vec::new();
    let cwd = env::current_dir().expect("Failed to obtain current working directory");
    for entry in WalkDir::new(cwd).follow_links(true) {
        let clean_entry = entry.expect("Failed to read from file system object");
        let path = clean_entry.into_path();
        let meta = fs::metadata(&path).unwrap();
        if meta.is_file() {
            let mod_time = meta
                .modified()
                .expect("Failed to get file modification time");
            files.push(File { path, mod_time })
        }
    }

    Ok(files)
}
