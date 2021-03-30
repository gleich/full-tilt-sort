use std::path::{Path, PathBuf};
use std::{env, fs, io};

use chrono::{DateTime, Local};
use walkdir::WalkDir;

/// A file in the structure
pub struct File {
	pub path: PathBuf,
	pub mod_time: DateTime<Local>,
}

impl File {
	/// Append the new folder path to the file name
	/// The new path is in the following pattern according to the modification date:
	/// Month/Day of the Week - Date/File Name
	/// An example of this would be March/Monday - 15/main.rs
	pub fn new_path(&self) -> PathBuf {
		let path = format!(
			"{}/{}",
			DateTime::format(&self.mod_time, "%B/%A - %e").to_string(),
			&self
				.path
				.file_name()
				.expect("Failed to get file name")
				.to_str()
				.unwrap()
		);
		Path::new(&path).to_path_buf()
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
