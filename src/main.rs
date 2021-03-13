mod structure;

fn main() {
    structure::get_files().expect("Failed to get files in structure");
}
