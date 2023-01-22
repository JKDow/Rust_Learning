/*
Crate challenge part 2 from: 
https://adventofcode.com/2022/day/5

Author: Josh Dowling
Created: 19/1/2023
Last updated: 19/1/2023 
*/

use advent_5_crates_2::{*, part_2::*}; 

fn main() {
    let _timer = program_timer::BenchMarker::new("Main");
    match run_part_2("input.txt") {
        Ok(s) => println!("{}",s),
        Err(err) => println!("{}", err),
    }
}

//attempt 1: CNSFCGJSM