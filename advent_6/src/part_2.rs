use std::char::from_digit;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::read_file_str;

pub fn run(path: &str) -> usize {
    let data = read_file_str(path); 

    return find_unique_hash(data)
}

fn find_unique(data: String) -> usize {
    let mut letters: HashMap<char, Vec<usize>> = HashMap::new();
    let letters_string = "abcdefghijklmnopqrstuvwxyz"; 
    let mut counter = 0; 
    let mut index = 0; 
    for letter in letters_string.chars() {
        letters.insert(letter, Vec::new());
    }
    let mut previous_match = [0,0]; 
    for char in data.chars() {
        let mut vec: Vec<usize> = letters.remove(&char).unwrap();
        vec.push(index);
        if vec.len() >= 2 {
            match vec.get(vec.len() - 2) {
                Some(previous_element) => {
                    if index - previous_element < 14 {
                        if previous_element > &previous_match[1] {
                            counter = index - previous_element;
                        } else {
                            if previous_match[0] < index - 14 {
                                counter = index - previous_match[1]; 
                            } else {
                                counter = index - previous_element;
                            }
                        }
                        previous_match[1] = index; 
                        previous_match[0] = *previous_element; 
                    } else {
                        counter += 1;
                    }
                }
                None => counter += 1,
            };
        } else {
            counter += 1;
        }
        letters.insert(char, vec);
        if counter == 14 {
            return index+1; 
        }
        index += 1;
    }

    return 0
}

fn find_unique_hash(data: String) -> usize {
    for i in 14..data.len()-14{
        let slice = &data[i-14..i];
        let set: HashSet<char> = HashSet::from_iter(slice.chars());
        if set.len() == 14 {
            return i;
        }
    }
    return 0
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let data = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let result = find_unique_hash(data);
        assert_eq!(result, 19)
    }

    #[test]
    fn data_2() {
        let data = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let result = find_unique_hash(data);
        assert_eq!(result, 23)
    }

    #[test]
    fn data_3() {
        let data = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let result = find_unique_hash(data);
        assert_eq!(result, 23)
    }

    #[test]
    fn data_4() {
        let data = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let result = find_unique_hash(data);
        assert_eq!(result, 29)
    }

    #[test]
    fn data_5() {
        let data = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let result = find_unique_hash(data);
        assert_eq!(result, 26)
    }
}