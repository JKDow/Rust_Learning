pub mod threads;

use crate::read_file_str;
use std::thread;
use threads::*;

pub fn run(input_path: &str) -> u32 {
    let mut messages = Message::new();
    thread::spawn(move || {
        update_viewable(messages.main_tx, messages.viewable_rx);
    });
    let directions:[Direction; 4] = [Direction::Bottom, Direction::Top, Direction::Right, Direction::Left];
    for i in 0..4 {
        let (tx, rx)= (messages.viewable_tx.pop().unwrap(), messages.side_rx.pop().unwrap());
        let dir = directions[i].clone();
        thread::spawn(move || {
            search_side(tx, rx, dir);
        });
    }
    let data_string = read_file_str(input_path);
    let mut rows: Vec<Vec<u32>> = vec![];
    let mut sent = false;
    for line in data_string.lines() {
        rows.push(line
            .chars()
            .map(|c| c
                .to_digit(10)
                .unwrap())
            .collect());
        if sent == false {
            messages.viewable_setup_tx.send(ViewableMsg::length_msg(rows[0].len())).unwrap();
            sent = true;
        }
        messages.side_tx[Direction::Left as usize].send(Some(rows[rows.len() - 1].clone())).unwrap();
        messages.side_tx[Direction::Right as usize].send(Some(rows[rows.len() - 1].clone())).unwrap();
    }
    messages.side_tx[Direction::Left as usize].send(None).unwrap();
    messages.side_tx[Direction::Right as usize].send(None).unwrap();
    for i in 0..rows.len() {
        let mut column: Vec<u32> = Vec::new();
        for j in 0..rows.len() {
            column.push(rows[j][i]);
        }
        messages.side_tx[Direction::Top as usize].send(Some(column.clone())).unwrap();
        messages.side_tx[Direction::Bottom as usize].send(Some(column)).unwrap();
    }
    messages.side_tx[Direction::Top as usize].send(None).unwrap();
    messages.side_tx[Direction::Bottom as usize].send(None).unwrap();
    let viewable_counter = messages.main_rx.recv().unwrap();
    return viewable_counter;
}
#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let result = run("input_test.txt");
        assert_eq!(result, 21)
    }
}