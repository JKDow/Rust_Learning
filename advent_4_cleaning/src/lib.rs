use std::io::BufReader;
use std::fs::File;

pub fn read_file(path: &str) -> BufReader<File> {
    let data = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            panic!("Could not open file"); 
        }
    }; 
    BufReader::new(data)
}

pub mod program_timer{
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
