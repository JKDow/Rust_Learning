/*
Calorie counter challenge from: 
https://adventofcode.com/2022/day/1

Author: Josh Dowling
Created: 11/1/2023
Last updated: 11/1/2023 
*/


use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    println!("Starting Main");

    //Read input to string
    let path = "input.txt";
    println!("Path: {}", path);

    let data = File::open(path).expect("Failed to open input file"); 
    let reader = BufReader::new(data); 

    let mut current_elf_count = 0; 
    let mut max_elf_count = 0 ;

    let mut i = 0; 
    let mut max_i = 0; 
    
    for line in reader.lines() {
        let l = line.expect("Couldn't read line");
        let num: u32 = match l.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //println!("Not a number"); 
                i += 1; 
                if current_elf_count > max_elf_count {
                    max_elf_count = current_elf_count;
                    max_i = i;
                }
                current_elf_count = 0;
                continue; 
            }
        };
        current_elf_count += num; 
        //println!("{}", l);
    }

    println!("Max Elf Count: {}", max_elf_count);
    println!("Max I: {}", max_i);
}
  