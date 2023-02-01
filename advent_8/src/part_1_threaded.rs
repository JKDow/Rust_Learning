pub mod threads;

use crate::read_file_str;
use std::thread;
use std::sync::mpsc;
use threads::*;

pub fn run(input_path: &str) -> u32 {
    let (viewable_tx, viewable_rx) = mpsc::channel::<ViewableMsg>();
    let viewable_left_tx = viewable_tx.clone();
    let viewable_right_tx = viewable_tx.clone();
    let viewable_top_tx = viewable_tx.clone();
    let viewable_bottom_tx = viewable_tx.clone();
    let (left_tx, left_rx) = mpsc::channel::<Option<Vec<u32>>>();
    let (right_tx, right_rx) = mpsc::channel::<Option<Vec<u32>>>();
    let (top_tx, top_rx) = mpsc::channel::<Option<Vec<u32>>>();
    let (bottom_tx, bottom_rx) = mpsc::channel::<Option<Vec<u32>>>();
    let (main_tx, main_rx) = mpsc::channel::<u32>();

    thread::spawn(move || {
        update_viewable(main_tx, viewable_rx);
    });

    thread::spawn(move || {
        search_side(viewable_left_tx, left_rx, Direction::Left);
    });
    thread::spawn(move || {
        search_side(viewable_right_tx, right_rx, Direction::Right);
    });
    thread::spawn(move || {
        search_side(viewable_top_tx, top_rx, Direction::Top);
    });
    thread::spawn(move || {
        search_side(viewable_bottom_tx, bottom_rx, Direction::Bottom);
    });

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
            viewable_tx.send(ViewableMsg::length_msg(rows[0].len())).unwrap();
            sent = true;
        }
        left_tx.send(Some(rows[rows.len() - 1].clone())).unwrap();
        right_tx.send(Some(rows[rows.len() - 1].clone())).unwrap();
    }
    left_tx.send(None).unwrap();
    right_tx.send(None).unwrap();
    for i in 0..rows.len() {
        let mut column: Vec<u32> = Vec::new();
        for j in 0..rows.len() {
            column.push(rows[j][i]);
        }
        top_tx.send(Some(column.clone())).unwrap();
        bottom_tx.send(Some(column)).unwrap();
    }
    top_tx.send(None).unwrap();
    bottom_tx.send(None).unwrap();

    let viewable_counter = main_rx.recv().unwrap();

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