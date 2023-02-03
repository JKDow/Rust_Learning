use crate::read_file_str;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up), 
            "D" => Some(Direction::Down),
            "R" => Some(Direction::Right),
            "L" => Some(Direction::Left),
            _ => None,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
enum PositionType {
    Same,
    Side (Direction),
    Corner (Direction, Direction),
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    last_move: (Option<Direction>, Option<Direction>),
    leader_position: PositionType,
    
}

impl Coordinate {
    fn new(x: i32, y:i32) -> Coordinate {
        Coordinate {
            x: x,
            y: y, 
            last_move: (None, None), 
            leader_position: PositionType::Same 
        }
    }

    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
        self.last_move = (Some(direction), None);
    }

    fn follow_head(&mut self, head: &Coordinate) {
        //Horizontal
        if head.x == self.x {
            if head.y > self.y + 1 {
                if head.y > self.y + 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction(Direction::Up);
                    return;
                }
            } else if head.y < self.y -1 {
                if head.y < self.y - 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction(Direction::Down);
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
                    self.move_direction(Direction::Right);
                    return;
                }
            } else if head.x < self.x -1 {
                if head.x < self.x - 2 {
                    panic!("Head too far");
                } else {
                    self.move_direction(Direction::Left);
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
            self.move_direction(Direction::Up);
            self.move_direction(Direction::Right);
            return;
        } else if (head.x == self.x - 2 && head.y == self.y + 1) || (head.x == self.x - 1 && head.y == self.y + 2) { //Top left
            self.move_direction(Direction::Up);
            self.move_direction(Direction::Left);
            return;
        } else if (head.x == self.x + 2 && head.y == self.y - 1) || (head.x == self.x + 1 && head.y == self.y - 2) { //Bottom right
            self.move_direction(Direction::Down);
            self.move_direction(Direction::Right);
            return;
        } else if (head.x == self.x - 2 && head.y == self.y - 1) || (head.x == self.x - 1 && head.y == self.y - 2) { //Bottom left
            self.move_direction(Direction::Down);
            self.move_direction(Direction::Left);
            return;
        } else {
            panic!("Head too far");
        }
    }  
}

pub fn run(input: &str) -> usize {
    let data_string = read_file_str(input);

    let mut visited: HashSet<Coordinate> = HashSet::new();

    let mut head = Coordinate::new(0, 0);
    let mut tail = Coordinate::new(0, 0);
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
            head.move_direction(Direction::from_str(dir).unwrap());
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