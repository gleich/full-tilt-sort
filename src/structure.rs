use std::path::{Path, PathBuf};
use std::{env, fs, io};

use chrono::{DateTime, Local};
use walkdir::WalkDir;

/// A file in the structure
pub struct File {
	path: PathBuf,
	mod_time: DateTime<Local>,
}

impl File {
	pub fn new_path(&self) -> PathBuf {
		// Generating parts
		let file_name = self.path.file_name().unwrap().to_str().unwrap();
		let parent = DateTime::format(&self.mod_time, "%B/%A - %e");

		// Put parts together
		Path::new(&format!("{}/{}", parent, file_name)).to_path_buf()
	}
}

/// Get all the file paths and their modification times
pub fn get_files() -> io::Result<Vec<File>> {
	let mut files: Vec<File> = Vec::new();
	let cwd = env::current_dir()?;

	for entry in WalkDir::new(cwd).follow_links(true) {
		let path = entry?.into_path();
		let meta = fs::metadata(&path).unwrap();

		if meta.is_file() {
			let mod_time = meta.modified()?;
			files.push(File {
				path,
				mod_time: mod_time.into(),
			})
		}
	}

	Ok(files)
}
