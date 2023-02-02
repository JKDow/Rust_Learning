use std::sync::mpsc;

#[derive(Clone)]
pub enum Direction {Left=0, Right=1, Top=2, Bottom=3}

pub struct Message {
    pub viewable_setup_tx: mpsc::Sender<ViewableMsg>,
    pub viewable_tx: Vec<mpsc::Sender<ViewableMsg>>,
    pub viewable_rx: mpsc::Receiver<ViewableMsg>,
    pub side_tx: Vec<mpsc::Sender<Option<Vec<u32>>>>,
    pub side_rx: Vec<mpsc::Receiver<Option<Vec<u32>>>>,
    pub main_tx: mpsc::Sender<u32>,
    pub main_rx: mpsc::Receiver<u32>,
}

impl Message {
    pub fn new() -> Message {
        let (v_tx, v_rx) = mpsc::channel::<ViewableMsg>();
        let mut viewable_tx: Vec<mpsc::Sender<ViewableMsg>> = vec![];
        let mut side_tx: Vec<mpsc::Sender<Option<Vec<u32>>>> = vec![];
        let mut side_rx: Vec<mpsc::Receiver<Option<Vec<u32>>>> = vec![];
        for _ in 0..4 {
            viewable_tx.push(v_tx.clone());
            let (s_tx, s_rx) = mpsc::channel::<Option<Vec<u32>>>();
            side_tx.push(s_tx);
            side_rx.push(s_rx);
        }
        let (main_tx, main_rx) = mpsc::channel::<u32>();
        Message {
            viewable_setup_tx: v_tx.clone(),
            viewable_tx: viewable_tx, 
            viewable_rx: v_rx, 
            side_tx: side_tx, 
            side_rx: side_rx, 
            main_tx: main_tx, 
            main_rx: main_rx,  
        }
    }
}

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
                let comp = match msg.direction {
                    Direction::Left | Direction::Right => &mut viewable[side_counters[msg.direction as usize]][i],
                    Direction::Top | Direction::Bottom => &mut viewable[i][side_counters[msg.direction as usize]],
                };
                if *comp == false {
                    *comp = true;
                    viewable_counter += 1;
                }
            }
        }
        side_counters[msg.direction as usize] += 1;
    }
    tx.send(viewable_counter).unwrap();
}
