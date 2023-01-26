use crate::read_file_str;

enum Side {Left, Right, Top, Bottom}

struct Forrest {
    heights: Vec<Vec<u32>>,
    viewable: Vec<Vec<bool>>,
    left_max: Vec<u32>,
    right_max: Vec<u32>,
    top_max: Vec<u32>,
    bottom_max: Vec<u32>,
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
        Forrest {
            heights: data,
            viewable: vec![vec![false; size]; size],
            left_max: vec![0; size],
            right_max: vec![0; size],
            top_max: vec![0; size],
            bottom_max: vec![0; size],
            data_size: size,
        }
    }

    fn is_viewable(&mut self, row: usize, column: usize, side: Side) -> bool {
        let mut max = match side {
            Side::Left => self.left_max[row],
            Side::Right => self.right_max[row],
            Side::Top => self.top_max[column],
            Side::Bottom => self.bottom_max[column],
        };
        self.check_viewable(row, column, &mut max)
    }

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