/*
Cleaning challenge part 2 from: 
https://adventofcode.com/2022/day/4

Author: Josh Dowling
Created: 18/1/2023
Last updated: 18/1/2023 
*/

use std::io::BufRead;
use advent_4_cleaning::*; 
fn main() {
    let _timer = program_timer::BenchMarker::new();
    let reader = read_file("input.txt");
    let mut total_contains = 0; 
    let mut total_overlap = 0; 

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                panic!("Could not read line"); 
            }
        };
        //Seperate line string into min, max 1 and 2
        let half: Vec<&str> = line.split(",").collect();
        let mut parts: Vec<u32> = Vec::new();
        for i in half.iter() {
            for j in i.split("-") {
                parts.push(
                    match j.parse() {
                        Ok(x) => x,
                        Err(_) => {
                            panic!("Could not parse"); 
                        }
                    }
                );
            }
        }
        //compare the two ranges - Contains
        if parts[0] < parts[2] { //range 1 min is less 
            //check is range 1 max is greater
            if parts[1] >= parts[3] {
                //range 1 holds range 2
                total_contains += 1; 
            }
        } else if parts[0] > parts[2] { //range 2 min is less
            //check is range 2 max is greater
            if parts[3] >= parts[1] {
                total_contains += 1;
            }
        }
        else { //min of each range is equal
            total_contains += 1;
        }

        // Overlapping ranges 
        if (parts[2] <= parts[1]) && (parts[0] <= parts[3]) {
            total_overlap += 1; 
        } 
    }
    println!("Total sets containing another: {}", total_contains);
    println!("Total overlaps: {}", total_overlap)

}
