use std::{fs::File, io::Read, path::Path};

pub fn load_file_contents(path: &Path) -> String {
    let mut file = File::open(path)
        .unwrap_or_else(|_| panic!("Unable to find file at path {}", path.to_str().unwrap()));

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).unwrap();

    file_contents
}
