mod input {
    use std::io::BufReader;
    use std::fs::File;
    struct InFile {
        file_path: String,
        reader: BufReader<File>,
    }

    impl InFile {
        fn new(file_path: String) -> Result<InFile, String> {
            let my_input: InFile = InFile { 
                file_path: file_path.clone(),
                reader: BufReader::new(
                    match File::open(file_path) {
                        Ok(f) => f,
                        Err(_) => {
                            return Err(String::from("Error opening file"))
                        }
                    }
                ),
            };
            Ok(my_input)
        }
    }
}