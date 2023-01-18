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

        let line_vector: Vec<char> = line.chars().collect(                  ); 
        if line_counter < 10 {
            for i in 0..=8 {
                if line_vector[i*4] == '[' {
                    crates[i].push(line_vector[(i*4)+1]); 
                }
            }
            continue; 
        } else if line_counter == 10 {
            //reverse vectors
            for i in 0..8 {
                crates[i].reverse();
            }
        }
    }

    println!("Array: {:?}", crates);
    
}

fn reverse_vector(mut input_vector: Vec<char>) -> Vec<char> {
    let mut temp_vector: Vec<char> = Vec::new(); 
    for _i in input_vector.clone() {
        temp_vector.push(
            match input_vector.pop() {
                Some(c) => c,
                None => {
                    println!("Could not pop"); 
                    return temp_vector; 
                }
            }
        ); 
    }
    temp_vector
}