use std::char::from_digit;

use crate::read_file_str;

pub fn run(path: &str) -> usize {
    let data = read_file_str(path); 

    return find_start(data)
}

fn find_start(data: String) -> usize {
    let mut set:[char; 4] = [from_digit(0,10).unwrap(); 4]; 
    let mut index = 0; 
    let mut countdown = 0; 

    for char in data.chars() {
        println!("Index: {}, Char: {}", index, char);
        println!("Countdown: {}", countdown);
        println!("{:?}", set);
        if !set.contains(&char) {
            if set[3] != '0' && countdown == 0{
                return index + 1; 
            }
        } else {
            let duplicate_pos = set.iter().position(|&x| x==char).unwrap(); 
            let current_pos = index % set.len();
            println!("Duplicate: {}, Current: {}", duplicate_pos, current_pos);
            let mut expected_countdown = 0; 
            if duplicate_pos == current_pos {
                if set[3] != '0' && set[current_pos] != char{
                    return index + 1; 
                }
            } else if duplicate_pos > current_pos { //Check countdown logic
                expected_countdown = duplicate_pos - current_pos;
            } else {
                expected_countdown = (4 - current_pos) + duplicate_pos + 1; 
            }
            if expected_countdown > countdown {
                countdown = expected_countdown;
            }
        }
        set[index % set.len()] = char;
        index += 1; 
        if countdown > 0 {countdown -= 1};
    }
    return 0
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let data = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let result = find_start(data);
        assert_eq!(result, 7)
    }

    #[test]
    fn data_2() {
        let data = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let result = find_start(data);
        assert_eq!(result, 5)
    }

    #[test]
    fn data_3() {
        let data = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let result = find_start(data);
        assert_eq!(result, 7)
    }

    #[test]
    fn data_4() {
        let data = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let result = find_start(data);
        assert_eq!(result, 10)
    }

    #[test]
    fn data_5() {
        let data = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let result = find_start(data);
        assert_eq!(result, 11)
    }
}