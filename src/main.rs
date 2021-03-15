mod structure;

fn main() {
	let files = structure::get_files().expect("Failed to get files in structure");
	for file in files.iter() {
		let path = file.new_path();
		println!("{}", &path.to_str().unwrap());
	}
}
