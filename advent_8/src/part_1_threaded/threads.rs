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

pub struct SearchThreadConfig {
    pub tx: mpsc::Sender<ViewableMsg>,
    pub rx: mpsc::Receiver<Option<Vec<u32>>>, 
    pub side: Direction,
}

impl SearchThreadConfig {
    pub fn new(tx: mpsc::Sender<ViewableMsg>, rx: mpsc::Receiver<Option<Vec<u32>>>, side: Direction) -> SearchThreadConfig{
        SearchThreadConfig { tx: tx, rx: rx, side: side }
    }
}

pub fn search_side(config: SearchThreadConfig) {
    let handle = thread::spawn(move || {
        println!("Starting Thread");
        let rx = config.rx;
        loop {
            let msg = match rx.recv().unwrap() {
                None => {
                    config.tx.send(ViewableMsg::finished_msg(config.side.clone())).unwrap();
                    break;
                }
                Some(msg) => msg,
            };
            println!("Received message: {:?}", msg);
            let mut set = ViewableMsg::new(config.side.clone(), msg.len());
            let index = match config.side {
                Direction::Left | Direction::Top => 0,
                Direction::Right | Direction::Bottom => msg.len() - 1,
            };
            let mut max = msg[index];
            set.data[index] = true;
            let mut counter = 1;
            for element in msg.iter().skip(1) {
                if *element > max {
                    max = *element;
                    set.data[counter] = true;
                }
                counter += 1;
            }
            config.tx.send(set).unwrap();
        }
    });
    handle.join().unwrap();
    println!("Ended Thread");
}