use crate::read_file_str;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum Side {Left, Right, Top, Bottom}

struct Forrest {
    heights: Vec<Vec<u32>>,
    viewable: Vec<Vec<bool>>,
    max: HashMap<Side, Vec<u32>>,
    data_size: usize,
}

impl Forrest {
    fn new(path: &str) -> Forrest {
        let data_string = read_file_str(path);
        let data: Vec<Vec<u32>> = data_string
            .lines()
            .map(|line| -> Vec<u32> {line
                .chars()
                .map(|c| c
                    .to_digit(10)
                    .unwrap())
                .collect()}
            )
            .collect();
        let size = data.len();
        let mut forrest = Forrest {
            heights: data,
            viewable: vec![vec![false; size]; size],
            max: HashMap::new(),
            data_size: size,
        };
        forrest.max.insert(Side::Left, vec![0; size]);
        forrest.max.insert(Side::Right, vec![0; size]);
        forrest.max.insert(Side::Top, vec![0; size]);
        forrest.max.insert(Side::Bottom, vec![0; size]);

        return forrest
    }

    fn is_viewable(&mut self, row: usize, column: usize, side: Side) -> bool {
        let mut max = self.max.remove(&side).unwrap();
        let mut ret = false;
        let max_index = match side {
            Side::Left => row,
            Side::Right => row,
            Side::Top => column,
            Side::Bottom => column,
        };
        if self.heights[row][column] > max[max_index] {
            max[max_index] = self.heights[row][column];
            if self.viewable[row][column] == true {
                ret = false; 
            } else {
                self.viewable[row][column] = true;
                ret = true;
            }
        }else if self.viewable[row][column] == false {
            if row == 0 || column == 0 || row == self.data_size-1 || column == self.data_size-1 {
                self.viewable[row][column] = true;
                ret = true;
            }
        }
        self.max.insert(side, max);
        return ret;
    }

}

pub fn run(input: &str) -> u32 {
    let mut forrest = Forrest::new(input);
    let mut viewable_counter = 0; 
    for i in 0..forrest.data_size {
        for j in 0..forrest.data_size {
            if forrest.max.get(&Side::Left).unwrap()[i] < 9 {
                if forrest.is_viewable(i, j, Side::Left) {
                    viewable_counter += 1;
                }
            }
            if forrest.max.get(&Side::Right).unwrap()[i] < 9 {
                if forrest.is_viewable(i, forrest.data_size-j-1, Side::Right) {
                    viewable_counter += 1;
                }
            }
            if forrest.max.get(&Side::Bottom).unwrap()[j] < 9 {
                if forrest.is_viewable(j, i, Side::Top)  {
                    viewable_counter += 1;
                }
            }
            if forrest.max.get(&Side::Bottom).unwrap()[j] < 9 {
                if forrest.is_viewable(forrest.data_size-j-1, i, Side::Bottom) {
                    viewable_counter += 1;
                }
            }
        }
    }
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