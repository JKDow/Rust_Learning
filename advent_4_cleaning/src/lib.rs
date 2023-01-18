use std::io::BufReader;
use std::fs::File;
use std::time::Instant;

fn read_file(path: String) -> BufReader<File> {
    let data = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            panic!("Could not open file"); 
        }
    }; 
    BufReader::new(data)
}

struct BenchMarker {
    start: Instant,
    end: Duration,
}

fn start(){
    println!("Startint Main");
    let start = Instant::now();
}