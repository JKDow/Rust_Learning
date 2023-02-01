use std::sync::mpsc;
use std::thread;

#[derive(Clone)]
pub enum Direction {Left, Right, Top, Bottom}

pub struct ViewableMsg {
    pub data: Vec<bool>,
    pub direction: Direction,
    pub length: usize,
    pub finished: bool,
}

impl ViewableMsg {
    pub fn new(direction: Direction, length: usize) -> ViewableMsg {
        ViewableMsg {
            data: vec![false; length],
            direction: direction,
            length: length,
            finished: false,
        }
    }

    pub fn length_msg(length: usize) -> ViewableMsg {
        ViewableMsg { data: vec![], direction: Direction::Left, length: length, finished: false}
    }

    pub fn finished_msg(direction: Direction) -> ViewableMsg {
        ViewableMsg { data: vec![], direction: direction, length: 0, finished: true}
    }
}

pub fn search_side(tx: mpsc::Sender<ViewableMsg>, rx: mpsc::Receiver<Option<Vec<u32>>>, side: Direction) {
    loop {
        let msg = match rx.recv().unwrap() {
            None => {
                tx.send(ViewableMsg::finished_msg(side.clone())).unwrap();
                break;
            }
            Some(msg) => msg,
        };
        let mut set = ViewableMsg::new(side.clone(), msg.len());
        let index = match side {
            Direction::Left | Direction::Top => 0,
            Direction::Right | Direction::Bottom => msg.len() - 1,
        };
        let mut max = msg[index];
        set.data[index] = true;
        let mut counter = 1;
        for element in msg.iter().skip(1) { //reverse for top and bot
            if *element > max {
                max = *element;
                set.data[counter] = true; // set.data[msg.len() - 1-counter] = true;
            }
            counter += 1;
        }
        tx.send(set).unwrap();
    }
}