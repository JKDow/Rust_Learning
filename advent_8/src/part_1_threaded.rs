use crate::read_file_str;
use std::thread;
use std::sync::mpsc;

pub mod threads;

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
        let mut left = true;
        let mut right = true;
        let mut top = true;
        let mut bottom = true;
        let mut left_counter = 0;
        let mut right_counter = 0;
        let mut top_counter = 0;
        let mut bottom_counter = 0;
        let setup_msg = viewable_rx.recv().unwrap();
        let mut viewable: Vec<Vec<bool>> = vec![vec![false; setup_msg.length]; setup_msg.length];
        let mut viewable_counter = 0;

        while left || right || top || bottom {
            let msg = viewable_rx.recv().unwrap();
            if msg.finished == true {
                match msg.direction {
                    Direction::Left => left = false,
                    Direction::Right => right = false,
                    Direction::Top => top = false,
                    Direction::Bottom => bottom = false,
                }
                continue;
            }
            match msg.direction {
                Direction::Left => {
                    let mut counter = 0;
                    for element in msg.data.iter() {
                        if *element == true {
                            if viewable[left_counter][counter] == false {
                                viewable_counter += 1;
                                viewable[left_counter][counter] = true;
                            }
                        }
                        counter += 1;
                    }
                    left_counter += 1;
                }
                Direction::Right => {
                    let mut counter = 0;
                    for element in msg.data.iter() {
                        if *element == true {
                            if viewable[right_counter][counter] == false {
                                viewable_counter += 1;
                                viewable[right_counter][counter] = true;
                            }
                        }
                        counter += 1;
                    }
                    right_counter += 1;
                }
                Direction::Top => {
                    let mut counter = 0;
                    for element in msg.data.iter() {
                        if *element == true {
                            if viewable[counter][top_counter] == false {
                                viewable_counter += 1;
                                viewable[counter][top_counter] = true;
                            }
                        }
                        counter += 1;
                    }
                    top_counter += 1;
                }
                Direction::Bottom => {
                    let mut counter = 0;
                    for element in msg.data.iter() {
                        if *element == true {
                            if viewable[counter][bottom_counter] == false {
                                viewable_counter += 1;
                                viewable[counter][bottom_counter] = true;
                            }
                        }
                        counter += 1;
                    }
                    bottom_counter += 1;
                }           
            }
        }
        main_tx.send(viewable_counter).unwrap();
    });

    thread::spawn(move || {
        threads::search_side(viewable_left_tx, left_rx, Direction::Left);
    });
    thread::spawn(move || {
        threads::search_side(viewable_right_tx, right_rx, Direction::Right);
    });
    thread::spawn(move || {
        threads::search_side(viewable_top_tx, top_rx, Direction::Top);
    });
    thread::spawn(move || {
        threads::search_side(viewable_bottom_tx, bottom_rx, Direction::Bottom);
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