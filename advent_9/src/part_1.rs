use crate::read_file_str;
use std::collections::HashSet;
use std::ops::Sub;
use std::cmp::Ordering;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    fn move_point(&mut self, direction: &str) { //Add diagonsl movement 
        match direction {
            "U" => self.y += 1,
            "D" => self.y -= 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => panic!("Invalid direction"),
        }
    }

    fn follow(&mut self, leader: &Point) -> bool{
        let difference = leader.clone() - self.clone();
        let touching = -1..=1;
        if touching.contains(&difference.x) && touching.contains(&difference.y) {
            return false;
        }
        match difference.x.cmp(&0) { //swap to if statements and compile horizontal and vertical movement into 1 string
            Ordering::Equal => {},
            Ordering::Less => {
                self.move_point("L");
            }
            Ordering::Greater => {
                self.move_point("R");
            }
        }
        match difference.y.cmp(&0) {
            Ordering::Equal => {},
            Ordering::Less => {
                self.move_point("D");
            }
            Ordering::Greater => {
                self.move_point("U");
            }
        }
        return true;
    } 
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

pub fn run(input: &str) -> usize {
    let data_string = read_file_str(input);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    visited.insert(tail.clone());

    for line in data_string.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let steps = split
            .next().unwrap()
            .parse::<usize>().unwrap();
        for _ in 0..steps {
            head.move_point(direction);
            if tail.follow(&head) {
                visited.insert(tail.clone());
            }
        }
    }
    return visited.len();
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let result = run("input_test.txt");
        assert_eq!(result, 13)
    }
}