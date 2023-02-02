use crate::read_file_str;

fn str_to_vec(input: String) -> Vec<Vec<u32>> {
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| -> Vec<u32> {line
            .chars()
            .map(|c| c
                .to_digit(10)
                .unwrap())
            .collect()}
        )
        .collect();
    return data;
}

fn calculate_score(row: usize, column: usize, data: &Vec<Vec<u32>>) -> u32 {
    let length = data.len();
    if row == 0 || column == 0 || row == length - 1 || column == length - 1 {
        return 0;
    }
    let height = data[row][column];
    let (mut left, mut right, mut up, mut down) = (0, 0, 0, 0);
    for i in (0..column).rev() { //Left
        if data[row][i] >= height {
            left += 1;
            break;
        }
        left += 1;
    }
    for i in column+1..length { //Right
        if data[row][i] >= height {
            right += 1;
            break;
        }
        right += 1;
    }
    for i in (0..row).rev() { //Up
        if data[i][column] >= height {
            up += 1;
            break;
        }
        up += 1;
    }
    for i in row+1..length { //Down
        if data[i][column] >= height {
            down += 1;
            break;
        }
        down += 1;
    }
    return left * right * up * down;
}

pub fn run(input: &str) -> u32 {
    let data_string = read_file_str(input);
    let data = str_to_vec(data_string);
    let mut max_score = 0;

    for row in 0..data.len() {
        for column in 0..data.len() {
            let score = calculate_score(row, column, &data);
            if score > max_score {
                max_score = score;
            }
        }
    }
    return max_score;
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let result = run("input_test.txt");
        assert_eq!(result, 8)
    }
}