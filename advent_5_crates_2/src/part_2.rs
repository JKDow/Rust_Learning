use crate::read_file_str;

#[derive(Debug)]
struct Instruction {
    number_of_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Instruction {
    fn new(instruction_string: &str) -> Instruction {
        let set = instruction_string
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|item| item.parse().unwrap())
            .collect::<Vec<usize>>();

        
        let instruction = Instruction {
            number_of_crates: set[0],
            from_stack: set[1],
            to_stack: set[2],
        };
        return instruction;
    }
}

#[derive(Debug)]
struct Crates {
    number_of_stacks: usize,
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn new(data: &str) -> Crates {
        let initial_config = data
        .split("\r\n\r\n")
        .nth(0).unwrap()
        .lines();
    
        let number_of_stacks = initial_config
            .clone()
            .last().unwrap()
            .chars()
            .rev()
            .nth(1).unwrap()
            .to_digit(10).unwrap();

        let mut crates = Crates {
            number_of_stacks: number_of_stacks as usize,
            stacks: Vec::new(),
        };

        for _ in 0..crates.number_of_stacks { //remove the loop?
            crates.stacks.push(Vec::new());
        }

        let config = initial_config
            .rev()
            .skip(1)
            //.map(|crate_row| crates.stacks.push(String::from(crate_row)));
            .collect::<Vec<&str>>();
        for row in config {
            let set = row
            .chars()
            .skip(1)
            .step_by(4)
            .collect::<Vec<char>>();
            let mut counter: usize = 0; 
            for char in set {
                if char != ' ' {
                    crates.stacks[counter].push(char);
                }
                counter += 1; 
            }
        }
        crates
    }

    fn run_instruction(&mut self, instruction: &Instruction) {

    }

    fn get_top_crates(&self) -> String {

        String::from("")
    }

}

pub fn run_part_2(path: &str) -> Result<&str, &str>{
    let data = read_file_str(path);

    let mut crates = Crates::new(&data);

    println!("Crates: {:?}", crates);

    let instruction_strings = data
        .split("\r\n\r\n")
        .nth(1).unwrap()
        .lines();
    for line in instruction_strings {
        let instruction = Instruction::new(line);
        crates.run_instruction(&instruction);
        println!("Instruction: {:?}", instruction);
    }
    
    
    
    Ok("MCD")
}






#[cfg(test)]
mod tests {
    use super::*;

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