/*
Crate challenge part 1 from: 
https://adventofcode.com/2022/day/5

Author: Josh Dowling
Created: 18/1/2023
Last updated: 18/1/2023 
*/

use std::io::BufRead;
use advent_5_crates::*; 

fn main() {
    let _timer = program_timer::BenchMarker::new();
    let reader = read_file("input.txt");

    let mut line_counter = 0; 

    let mut crates : [Vec<char>; 9] = core::array::from_fn(|_| Vec::new());

    for line in reader.lines() {
        line_counter += 1;

        let line = match line {
            Ok(line) => line,
            Err(_) => {
                panic!("Could not read line"); 
            }
        };
        // populate crates vectors
        //println!("Counter: {}", line_counter);
        if line_counter < 10 {
            let line_vector: Vec<char> = line.chars().collect(); 
            for i in 0..=8 {
                if line_vector[i*4] == '[' {
                    crates[i].push(line_vector[(i*4)+1]); 
                }
            }
            continue; 
        } else if line_counter == 10 {
            //reverse vectors
            println!("Stack start: {:?}", crates);
            for i in 0..=8 {
                crates[i].reverse();
            }
            println!("Reversed: {:?}", crates);
            continue; 
        }
        // get instructions
        let mut instruction: [u32; 3] = [0,0,0];  
        let mut counter = 0;
        for block in line.split_whitespace(){
            match block.parse::<u32>() {
                Ok(num) => {
                    instruction[counter] = num;
                    counter += 1; 
                },
                Err(_) => {continue;},
            };
        }
        //execute instruction
        //println!("Stack {}: {:?}",line_counter, crates);
        //println!("Instruction: {:?}", instruction);
        println!("");
        for _i in 0..instruction[0] {
            let temp: char = match crates[(instruction[1]-1) as usize].pop() {
                Some(temp) => temp,
                None => {
                   panic!("Invalid instruction: {}", line_counter);
                }
            };
            crates[(instruction[2]-1) as usize].push(temp);
        }
    }
    println!("Stack fished: {:?}", crates);
}

/*Wrong: 
CNTCFZSJH
CNZLFZSJH
*/

// RNZLFZSJH