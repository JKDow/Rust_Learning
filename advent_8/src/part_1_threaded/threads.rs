use std::sync::mpsc;

#[derive(Clone, Debug)]
pub enum Direction {Left=0, Right=1, Top=2, Bottom=3}

#[derive(Debug)]
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
        match side {
            Direction::Left | Direction::Top => {
                let mut max = 0;
                for i in 0..=msg.len()-1 { //reverse for top and bot
                    if msg[i] > max || i==0 {
                        max = msg[i];
                        set.data[i] = true; // set.data[msg.len() - 1-counter] = true;
                    }
                }
            }
            Direction::Right | Direction::Bottom => {
                let mut max = 0;
                for i in (0..=msg.len()-1).rev() { //reverse for top and bot
                    if msg[i] > max || i == msg.len()-1{
                        max = msg[i];
                        set.data[i] = true; // set.data[msg.len() - 1-counter] = true;
                    }
                }
            }
        };

        tx.send(set).unwrap();
    }
}

pub fn update_viewable(tx: mpsc::Sender<u32>, rx: mpsc::Receiver<ViewableMsg>) {
    let mut run = 4;
    let mut side_counters: [usize; 4] = [0; 4]; //left, right, top, bottom
    let setup_msg = rx.recv().unwrap();
    let mut viewable: Vec<Vec<bool>> = vec![vec![false; setup_msg.length]; setup_msg.length];
    let mut viewable_counter = 0;

    while run > 0 {
        let msg = rx.recv().unwrap();
        if msg.finished == true {
            run -= 1;
            continue;
        }
        for i in 0..msg.data.len() {
            if msg.data[i] {
                if match msg.direction {
                    Direction::Left | Direction::Right => compare_row(&mut viewable, side_counters[msg.direction as usize], i),
                    Direction::Top | Direction::Bottom => compare_column(&mut viewable, side_counters[msg.direction as usize], i),
                } { 
                    viewable_counter += 1;
                }
            }
        }
        side_counters[msg.direction as usize] += 1;
    }
    tx.send(viewable_counter).unwrap();
}

fn compare_row(viewable: &mut Vec<Vec<bool>>, counter: usize, index: usize) -> bool {
    if viewable[counter][index] == false {
        viewable[counter][index] = true;
        return true;
    } 
    return false;
}

fn compare_column(viewable: &mut Vec<Vec<bool>>, counter: usize, index: usize) -> bool {
    if viewable[index][counter] == false {
        viewable[index][counter] = true;
        return true;
    } 
    return false;
}

fn display_viewable(viewable: &Vec<Vec<bool>>) {
    for x in viewable.iter() {
        println!("{:?}", x);
    }
}