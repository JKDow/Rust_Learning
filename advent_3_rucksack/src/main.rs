/*
Rock paper scissors challenge part 3 from: 
https://adventofcode.com/2022/day/3

Author: Josh Dowling
Created: 15/1/2023
Last updated: 15/1/2023 
*/

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    println!("Starting Main");
    let start = Instant::now();
    
    let priorities = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"); 
    let priorities: Vec<char> = priorities.chars().collect(); 
    let mut total_sum = 0;

    // Read input.txt file into buf reader object called reader 
    let path = "input.txt"; 
    let data = File::open(path).expect("Failed to open input file"); 
    let reader = BufReader::new(data); 

    // Iterate through lines
    for line in reader.lines() {
        let line = match line {
            Ok(x) => x,
            Err(_) => {
                println!("Could not read line"); 
                return; 
            }
        };

        // Read line into vector and slice the vector in two 
        let whole: Vec<char> = line.chars().collect();
        let vec_length = whole.len(); 
        //println!("New Line Vector {:?}", whole);

        let sack1 = &whole[..(vec_length/2)]; 
        let sack2 = &whole[(vec_length/2)..];
        //println!("Sack 1 vector: {:?}", sack1);
        //println!("Sack 2 vector: {:?}", sack2);

        //Pass each vector into a hashset 
        let sack_map_1: HashSet<char> = sack1.iter().cloned().collect();
        let sack_map_2: HashSet<char> = sack2.iter().cloned().collect();
        //println!("HashSet 1: {:?}", sack_map_1); 
        //println!("HashSet 2: {:?}", sack_map_2); 

        // Check if there are matching elements in hashSet 
        for item in sack_map_1.intersection(&sack_map_2) {
            let mut counter = 0; 
            for x in &priorities {
                counter += 1; 
                if item == x {
                    //println!("Match: {}, Counter: {}", x, counter);
                    total_sum += counter; 
                    break; 
                }
            }
        }
    }
    println!("Total: {}", total_sum); 

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

//7682 too low