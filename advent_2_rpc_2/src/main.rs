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

fn main() {
    println!("Starting Main");

    let rock = Move {
        points: 1,
        win: String::from("C"),
        lose: String::from("B"),
    };

    let paper = Move {
        points: 2,
        win: String::from("A"),
        lose: String::from("C"),
    };

    let scissors = Move {
        points: 3,
        win: String::from("B"),
        lose: String::from("A"),
    };
    
    let mut str_to_move = HashMap::new();
    str_to_move.insert("A".to_string(), rock); 
    str_to_move.insert("B".to_string(), paper);
    str_to_move.insert("C".to_string(), scissors); 
}
