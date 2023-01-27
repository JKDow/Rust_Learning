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
            .map(|line| line
                .chars()
                .map(|c| c
                    .to_digit(10)
                    .unwrap())
                .collect()
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

        let max_index = match side {
            Side::Left => row,
            Side::Right => row,
            Side::Top => column,
            Side::Bottom => column,
        };

        if self.heights[row][column] > max[max_index] {
            max[max_index] = self.heights[row][column];
            if self.viewable[row][column] == true {
                self.max.insert(side, max);
                return false; 
            }
            self.viewable[row][column] = true;
            self.max.insert(side, max);
            return true;
        }
        if row == 0 || column == 0 || row == self.data_size-1 || column == self.data_size-1 {
            if self.viewable[row][column] == true {
                self.max.insert(side, max);
                return false; 
            }
            self.viewable[row][column] = true;
            self.max.insert(side, max);
            return true;
        }
        self.max.insert(side, max);
        return false;
    }

    /*fn is_viewable(&mut self, row: usize, column: usize, side: Side) -> bool {
        let &mut max = match side {
            Side::Left => &mut self.left_max[row],
            Side::Right => &mut self.right_max[row],
            Side::Top => &mut self.top_max[column],
            Side::Bottom => &mut self.bottom_max[column],
        };
        self.check_viewable(row, column, max)
    }
    */
    fn check_viewable(&mut self, row: usize, column: usize, max: &mut u32) -> bool {
        if self.heights[row][column] > *max {
            *max = self.heights[row][column];
            if self.viewable[row][column] == true {
                return false; 
            }
            self.viewable[row][column] = true;
            return true;
        }
        if row == 0 || column == 0{
            if self.viewable[row][column] == true {
                return false; 
            }
            self.viewable[row][column] = true;
            return true;
        }
        return false;
    }
}

pub fn run(input: &str) -> u32 {
    let mut forrest = Forrest::new(input);
    let mut viewable_counter = 0; 
    for i in 0..forrest.data_size {
        for j in 0..forrest.data_size {
            //check row
            if forrest.is_viewable(i, j, Side::Left) {
                viewable_counter += 1;
            }

            if forrest.is_viewable(i, forrest.data_size-j-1, Side::Right) {
                viewable_counter += 1;
            }
            //check column 
            if forrest.is_viewable(j, i, Side::Top)  {
                viewable_counter += 1;
            }
            if forrest.is_viewable(forrest.data_size-j-1, i, Side::Bottom) {
                viewable_counter += 1;
            }
        }
    }

    for row in forrest.viewable {
        println!("{:?}", row);
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