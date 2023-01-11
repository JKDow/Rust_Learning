/*
Calorie counter challenge from: 
https://adventofcode.com/2022/day/1

Author: Josh Dowling
Created: 11/1/2023
Last updated: 11/1/2023 
*/

use std::fs;

fn main() {
    println!("Starting Main");

    //Read input to string
    let path = "input.txt";
    println!("Path: {}", path);

    let contents = fs::read_to_string(path)
        .expect("Error reading input file"); 
    
    //println!("{}",contents); 

    let mut current_line = String::new(); 
    let mut marker: bool = false; 
    let current_elf: Vec<i32> = Vec::new();
    let all_elfs: Vec<i32> = Vec::new();

    for i in contents.chars() {
        match i {
            '0'..='9' => {
                current_line.push(i);
                println!("{}",i); 
            }
            '\n' => {
                if marker {
                
                }
                else {
                    marker = true;
                }
            }
            _ => {
                println!("Invalid character: {}", i);
                return; 
            }
        }
    }

}
  