use crate::read_file_str;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn move_direction(&mut self, direction: &str) {
        match direction {
            "L" => self.x -= 1,
            "R" => self.x += 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    fn follow_head(&mut self, head: &Coordinate) {
        //Horizontal
        if head.x == self.x {
            if head.y > self.y + 1 {
                if head.y > self.y + 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction("U");
                    return;
                }
            } else if head.y < self.y -1 {
                if head.y < self.y - 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction("D");
                    return;
                }
            } else {
                return;
            }
        }
        //Vertical
        if head.y == self.y {
            if head.x > self.x + 1 {
                if head.x > self.x + 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction("R");
                    return;
                }
            } else if head.x < self.x -1 {
                if head.x < self.x - 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction("L");
                    return;
                }
            } else {
                return;
            }
        }
        //Diagonals
        if (head.x == self.x + 1 && head.y == self.y + 1) || 
            (head.x == self.x + 1 && head.y == self.y - 1) || 
            (head.x == self.x - 1 && head.y == self.y + 1) || 
            (head.x == self.x - 1 && head.y == self.y - 1) {
            return;
        }
        if (head.x == self.x + 2 && head.y == self.y + 1) || (head.x == self.x + 1 && head.y == self.y + 2) { // Top right
            self.move_direction("U");
            self.move_direction("R");
            return;
        } else if (head.x == self.x - 2 && head.y == self.y + 1) || (head.x == self.x - 1 && head.y == self.y + 2) { //Top left
            self.move_direction("U");
            self.move_direction("L");
            return;
        } else if (head.x == self.x + 2 && head.y == self.y - 1) || (head.x == self.x + 1 && head.y == self.y - 2) { //Bottom right
            self.move_direction("D");
            self.move_direction("R");
            return;
        } else if (head.x == self.x - 2 && head.y == self.y - 1) || (head.x == self.x - 1 && head.y == self.y - 2) { //Bottom left
            self.move_direction("D");
            self.move_direction("L");
            return;
        } else {
            panic!("Head too far");
        }
    }
}

pub fn run(input: &str) -> usize {
    let data_string = read_file_str(input);

    let mut visited: HashSet<Coordinate> = HashSet::new();

    let mut head = Coordinate{x: 0, y: 0};
    let mut tail = Coordinate{x: 0, y: 0};
    visited.insert(tail.clone());

    for line in data_string.lines() {
        let dir = line
            .split_whitespace()
            .nth(0).unwrap();
        let steps: u32 = line
            .split_whitespace()
            .nth(1).unwrap()
            .parse().unwrap();
        for _ in 0..steps {
            head.move_direction(dir);
            tail.follow_head(&head);
            visited.insert(tail.clone());
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