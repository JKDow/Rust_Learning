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
use std::time::Instant;

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
    let start = Instant::now();

    let mut str_to_move = HashMap::new();
    str_to_move.insert("A".to_string(), Move::new(1, "C", "B")); // A = rock
    str_to_move.insert("B".to_string(), Move::new(2, "A", "C")); // B = paper
    str_to_move.insert("C".to_string(), Move::new(3, "B", "A")); // C = Scissors

    let path = "input.txt";
    let data = File::open(path).expect("Failed to open input file"); 
    let reader = BufReader::new(data); 

    let mut points = 0;

    for line in reader.lines() {
        let mut v: Vec<&str> = Vec::new();
        let l = line.expect("Couldn't read line");
        for byte in l.split_whitespace(){
            v.push(byte); 
        }
        let oponent_move = str_to_move.get(v[0]).unwrap(); 
        match v[1] {
            "X" => { //Lose
                points += str_to_move.get(&oponent_move.win).unwrap().points; 
            },
            "Y" => { // Draw
                points += oponent_move.points + 3; 
            },
            "Z" => { // Win
                points+= str_to_move.get(&oponent_move.lose).unwrap().points + 6; 
            },
            _ => {},
        }
        //println!("Points: {}", points);
    }

    println!("Total Points: {}", points);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
