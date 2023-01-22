
struct directory {
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    files: Vec<File>,
    total_size: usize,
}

struct File {
    name: String,
    size: usize,
}

pub fn run(data: &str) -> usize {
    


    return 0
}



#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let result = run("input.txt");
        assert_eq!(result, 95437)
    }
}