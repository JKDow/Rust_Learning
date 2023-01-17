/*
Rucksack challenge part 2 from: 
https://adventofcode.com/2022/day/3

Author: Josh Dowling
Created: 17/1/2023
Last updated: 17/1/2023 
*/

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

fn main() {
    println!("Startint Main");
    let start = Instant::now();

    let priorities = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"); 
    let priorities: Vec<char> = priorities.chars().collect(); 
    let mut total_sum = 0;

    let mut groups = 0; 

    // Take input from file 
    let path = "input.txt"; 
    let data = File::open(path).expect("Failed to open input file"); 
    let reader = BufReader::new(data); 

    let mut counter = 0; 
    let mut compare_data: Vec<char> = Vec::new(); 
    // Read 3 lines into vector of strings
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if counter == 0 {
            compare_data = line.chars().collect(); 
            counter += 1; 
            continue;
        }
        compare_data = comp(&compare_data, &line); 
        counter += 1; 

        if counter == 3 {
            groups += 1; 
            // Add priority to total 
            counter = 0; 
            for item in priorities.iter() {
                counter += 1; 
                if *item == compare_data[0] {
                    total_sum += counter; 
                    println!("Group: {}, Item: {:?}, Counter: {}, Total: {}", groups, compare_data, counter, total_sum);
                }
            }
            counter = 0; 
            continue; 
        }
    }
    println!("Total: {}", total_sum);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn comp(set1: &Vec<char>, set2: &str) -> Vec<char> {
    let mut data: Vec<char> = Vec::new();
    for item in set1.iter() {
        if set2.contains(*item){
            data.push(*item)
        }
    }
    data
}