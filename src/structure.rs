use std::path::PathBuf;
use std::time::SystemTime;
use std::{env, fs};

use walkdir::WalkDir;

/// A file in the structure
pub struct File {
	path: PathBuf,
	mod_time: SystemTime,
}

/// Get all the file paths and their modification times
pub fn get_files() -> std::io::Result<Vec<File>> {
	let mut files: Vec<File> = Vec::new();
	let cwd = env::current_dir()?;

	for entry in WalkDir::new(cwd).follow_links(true) {
		let path = entry?.into_path();
		let meta = fs::metadata(&path).unwrap();

		if meta.is_file() {
			let mod_time = meta.modified()?;
			files.push(File { path, mod_time })
		}
	}

	Ok(files)
}
