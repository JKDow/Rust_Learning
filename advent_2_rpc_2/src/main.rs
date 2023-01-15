/*
Rock paper scissors challenge part 2 from: 
https://adventofcode.com/2022/day/2

Author: Josh Dowling
Created: 13/1/2023
Last updated: 13/1/2023 
*/

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

struct Move {
    points: u32,
    win: String,
    lose: String,
}

impl Move {
    fn new(points: u32, win: &str, lose: &str) -> Move {
        Move { 
            points,
            win: String::from(win),
            lose: String::from(lose),
        }
    }
}

fn main() {
    println!("Starting Main");

    let mut str_to_move = HashMap::new();
    str_to_move.insert("A".to_string(), Move::new(1, "C", "B")); // A = rock
    str_to_move.insert("B".to_string(), Move::new(2, "A", "C")); // B = paper
    str_to_move.insert("C".to_string(), Move::new(3, "B", "C")); // C = Scissors

    
}
