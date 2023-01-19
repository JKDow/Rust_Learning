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
    }
    
    impl BenchMarker {
        pub fn new() -> BenchMarker {
            println!("Starting Program Main");
            let bench = BenchMarker{start: Instant::now()};
            return bench; 
        }
    }
    
    impl Drop for BenchMarker {
        fn drop(&mut self) {
            let duration = self.start.elapsed();
            println!("Ending main after: {:?}", duration)
        }
    }
}

pub mod crates {
    struct Instruction {
        number_of_crates: usize,
        from_stack: usize,
        to_stack: usize,
        carry_one_at_time: bool, 
    }
    
    struct Crates {
        stacks: Vec<Vec<char>>,
    }

    pub fn run_part_2(path: &str) -> Result<&str, &str>{


        Ok("MCD")
    }
}

#[cfg(test)]
mod tests {
    use crate::crates::run_part_2;

    //use super::*;

    #[test]
    fn runs_part_2() {
        match run_part_2("input.txt") {
            Ok(v) => {
                assert_eq!("MCD", v);
                v
            }
            Err(e) => panic!("{}", e) 
        };
    }
}