use crate::read_file_str;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Direction {
    U, D, L, R, UR, DR, UL, DL
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        use Direction::*;
        match s {
            "U" => Some(U), 
            "D" => Some(D),
            "R" => Some(R),
            "L" => Some(L),
            _ => None,
        }
    }

    fn combine(&self, other: Direction) -> Option<Direction> {
        use Direction::*;
        if self.opposite() == other {
            return None;
        } else if (*self == U && other == R) || (*self == R && other == U) {
            return Some(UR);
        } else if *self == U && other == L || (*self == L && other == U){
            return Some(UL);
        } else if *self == D && other == R || (*self == R && other == D){
            return Some(DR);
        } else if *self == D && other == L || (*self == L && other == D){
            return Some(DL);
        } else {
            panic!("Invalid direction composite");
        }
    }

    fn split(&self) -> (Direction, Direction) {
        use Direction::*;
        match self {
            UR => (U, R),
            UL => (U, L), 
            DR => (D, R),
            DL => (D, L),
            _ => panic!("Invalid split")
        }
    }

    fn opposite (&self) -> Direction {
        use Direction::*;
        match self {
            U => D,
            D => U,
            L => R,
            R => L,
            UR => DL,
            UL => DR,
            DR => UL,
            DL => UR,
        }
    }

    fn is_straight(&self) -> bool {
        use Direction::*;
        if *self == U || *self == D || *self == L || *self == R {
            return true;
        }
        return false;
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Knot {
    coords: Coordinate,
    last_move: Option<Direction>,
    leader_position: Option<Direction>,
}

impl Knot {
    fn new(x: i32, y:i32) -> Knot {
        Knot {
            coords: Coordinate { x: x, y: y },
            last_move: None, 
            leader_position: None 
        }
    }

    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::L => self.coords.x -= 1,
            Direction::R => self.coords.x += 1,
            Direction::U => self.coords.y += 1,
            Direction::D => self.coords.y -= 1,
            Direction::UR => {
                self.coords.y += 1;
                self.coords.x += 1;
            }
            Direction::UL => {
                self.coords.y += 1;
                self.coords.x -= 1
            }
            Direction::DR => {
                self.coords.y -= 1;
                self.coords.x += 1;
            }
            Direction::DL => {
                self.coords.y -= 1;
                self.coords.x -= 1;
            }
        }
        self.last_move = Some(direction);
    }

    fn follow_leader(&mut self, lead: &Knot) {
        let position = match self.leader_position.clone() {
            Some(pos) => pos,
            None => { //leader was ontop of self
                self.leader_position = lead.last_move.clone(); //update to new position
                self.last_move = None;
                return;
            }
        };
        let last_move = match lead.last_move.clone() {
            Some(last_move) => last_move,
            None => { //Leader did not move 
                self.last_move = None;
                return;
            }
        };
        if position == last_move { //Leader moved in straight line away
            self.move_direction(last_move); //Follow the leader in straight line
            return;
        }
        if position.is_straight(){ //was on a side 
            if last_move.is_straight() {
                self.leader_position = position.combine(last_move);
                self.last_move = None; 
                return;
            } else { //Leader moved diagonally 
                let (last_1, last_2) = last_move.split();
                if position == last_1 {
                    self.leader_position = Some(last_1); 
                    self.move_direction(last_move); 
                    return;
                } else if position == last_2 { //Leader moved diagonally away so follow 
                    self.leader_position = Some(last_2); 
                    self.move_direction(last_move); 
                    return;
                } else { //leader moved diagonally but still touching 
                    if last_1 == position.opposite() {
                        self.leader_position = Some(last_2);
                    } else { 
                        self.leader_position = Some(last_1);
                    }
                    self.last_move = None;
                    return;
                }
            }
        } else { //was on a corner
            let (pos_1, pos_2) = position.split();
            if last_move.is_straight() {
                if last_move == pos_1 {
                    self.leader_position = Some(pos_1);
                    self.move_direction(position);
                    return;
                } else if last_move == pos_2 {
                    self.leader_position = Some(pos_2);
                    self.move_direction(position);
                    return;
                }
                if last_move.opposite() == pos_1 {
                    self.leader_position = Some(pos_2);
                    return;
                } else {
                    self.leader_position = Some(pos_1);
                    return;
                }
            }
            if last_move == position.opposite() {
                self.leader_position = None; 
                self.last_move = None;
                return;
            }
            let (last_1, last_2) = last_move.split();
            if last_1 == pos_1 || last_1 == pos_2 {
                self.leader_position = Some(last_1.clone());
                self.move_direction(last_1);
                return;
            } else {
                self.leader_position = Some(last_2.clone());
                self.move_direction(last_2);
                return;
            }
        }
    }
}

pub fn run(input: &str) -> usize {
    let data_string = read_file_str(input);

    let mut visited: HashSet<Coordinate> = HashSet::new();

    let mut head = Knot::new(0, 0);
    let mut tail = Knot::new(0, 0);
    visited.insert(tail.coords.clone());

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
            tail.follow_leader(&head);
            visited.insert(tail.coords.clone());
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