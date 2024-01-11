use std::path::PathBuf;
use std::path::Path;
use std::fs;

pub fn read_text_resource<P: AsRef<Path>>(rel_path: P) -> Vec<u8> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(rel_path);
    let contents = fs::read(file_path)
        .unwrap_or_else(|_| panic!("Could not read '{}'", stringify!(file_path)));

    contents
}