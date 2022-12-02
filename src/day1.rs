fn load_file(left: usize, right: usize) -> usize {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
