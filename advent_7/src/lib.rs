use std::io::BufReader;
use std::fs::{File, read_to_string};

pub fn read_file_buf(path: &str) -> BufReader<File> {
    let data = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            panic!("Could not open file"); 
        }
    }; 
    BufReader::new(data)
}

pub fn read_file_str(path: &str) -> String {
    match read_to_string(path) {
        Ok(data) => data,
        Err(_) => {
            panic!("Could not open file"); 
        }
    }
}

pub mod program_timer {
    use std::time::Instant;
    pub struct BenchMarker {
        start: Instant,
        name: String,
    }
    
    impl BenchMarker {
        pub fn new(name: &str) -> BenchMarker {
            println!("Starting function: Main");
            let bench = BenchMarker{
                start: Instant::now(),
                name: String::from(name)
            };
            return bench; 
        }
    }
    
    impl Drop for BenchMarker {
        fn drop(&mut self) {
            let duration = self.start.elapsed();
            println!("Ending {} after: {:?}", self.name, duration)
        }
    }
}

pub mod part_1;
pub mod part_2;



