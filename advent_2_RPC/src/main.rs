/*
Rock paper scissors challenge from: 
https://adventofcode.com/2022/day/2

Author: Josh Dowling
Created: 12/1/2023
Last updated: 12/1/2023 
*/

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
struct Game {
    my_move: Move,
    opponent_move: Move,
}

impl Game {
    fn new(my_move: Move, opponent_move: Move) -> Game {
        Game {
            my_move,
            opponent_move,
        }
    }
    
    fn play(&self) -> u32 {
        let mut _points:u32 = 0;
        match self.my_move{
            Move::Rock => {
                _points = 1; 
                match self.opponent_move {
                    Move::Rock => _points += 3,
                    Move::Paper => {}
                    Move::Scissors => _points += 6,
                }
            }
            Move::Paper => {
                _points = 2; 
                match self.opponent_move {
                    Move::Rock => _points += 6,
                    Move::Paper => _points += 3,
                    Move::Scissors => {}
                }
            }
            Move::Scissors => {
                _points = 3;
                match self.opponent_move {
                    Move::Rock => {},
                    Move::Paper => _points += 6,
                    Move::Scissors => _points += 3,
                }
            }
        }
        return _points
    }
}

fn main() {
    println!("Starting Main");

    let mut str_to_move = HashMap::new();
    str_to_move.insert("A".to_string(), Move::Rock); 
    str_to_move.insert("B".to_string(), Move::Paper);
    str_to_move.insert("C".to_string(), Move::Scissors); 
    str_to_move.insert("X".to_string(), Move::Rock);
    str_to_move.insert("Y".to_string(), Move::Paper);
    str_to_move.insert("Z".to_string(), Move::Scissors); 

    let path = "input.txt";
    let data = File::open(path).expect("Failed to open input file"); 
    let reader = BufReader::new(data); 

    let mut points = 0;
    for line in reader.lines() {
        let mut v: Vec<Move> = Vec::new();
        let l = line.expect("Couldn't read line");
        for byte in l.split_whitespace(){
            v.push(str_to_move.get(byte).unwrap().clone()); 
        }
        //println!("moves: {:?}, {:?}",v[0],v[1]);
        let my_game = Game::new(v[1], v[0]);
        points += my_game.play(); 
        //println!("Points: {}", points);
    }
    println!("Final Points: {}", points);
}
