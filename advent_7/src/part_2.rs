use crate::read_file_str;
use std::{collections::HashMap};


struct Directory {
    index: usize,

    parent: Option<usize>,
    children: HashMap<String, usize>,
    files: HashMap<String, u32>,
    total_size: u32,
}

impl Directory {
    fn new(index: usize, parent: Option<usize>) -> Self {
        let dir = Directory {
            index: index,
            parent: parent,
            children: HashMap::new(),
            files: HashMap::new(),
            total_size: 0, //will get updated when the search is done
        };
        return dir
    }

    fn add_child(&mut self, index: usize, name: String) -> Option<Directory> {
        //check if it exists
        if self.children.contains_key(&name) {
            return None;
        }
        //if not then make it
        self.children.insert(name.clone(), index);
        return Some(Directory::new(index, Some(self.index)))
    }

    fn add_file(&mut self, name: String, size: u32) {
        //check if it exists
        if self.files.contains_key(&name) {
            return;
        }
        //if not then make it
        self.files.insert(name, size);
    }

    fn sum_files(&mut self) -> u32 {
        let mut sum = 0;
        for (_, size) in &self.files {
            sum += size;
        }
        sum
    }
}

fn find_delete(mut nodes: Vec<Directory>) -> usize {
    let mut node = 0;
    let mut checked: Vec<usize> = Vec::new();
    let mut path: Vec<usize> = Vec::new();
    loop {
        let mut unchecked = 0;
        for (_, val) in &nodes[node].children { //find an unchecked child node 
            if !checked.contains(val) {
                unchecked = *val;
                break; 
            }
        }
        if unchecked > 0 { //this is the unchecked child
            path.push(node);
            node = unchecked;
        } else { // no children nodes unchecked
            let sum = get_children_size(&nodes, node) + nodes[node].sum_files(); //size of children + its files 
            checked.push(node); //add to checked 
            nodes[node].total_size = sum; //update total size

            node = match path.pop() { //update node to last element in path
                Some(x) => x,
                None => break,
            }
        }
    }
    let free_space = 70000000 - nodes[0].total_size;
    let to_free = 30000000 - free_space;
    
    let mut min = 70000000;

    for dir in nodes {
        if dir.total_size >= to_free {
            if dir.total_size < min {
                min = dir.total_size;
            }
        }
    }

    return min as usize;
}

fn get_children_size(nodes: &Vec<Directory>, index: usize) -> u32 {
    let mut result = 0;
    for (_, child) in &nodes[index].children {
        result += &nodes[*child].total_size;
    }
    return result
}

pub fn run(path: &str) -> usize {
    let mut nodes = Vec::new();
    let dir = Directory::new(nodes.len(), None);
    nodes.push(dir);
    let mut current_dir = 0;

    let data = read_file_str(path);
    for line in data.lines().skip(1) {
        let length = nodes.len();
        let slice = &line[..4];
        if slice == "$ cd" {
            let location = line.split_whitespace().nth(2).unwrap();
            if location == "/" {
                current_dir = 0; 
            } else if location == ".." {
                current_dir = nodes[current_dir].parent.unwrap();
            } else {
                current_dir = match nodes[current_dir].children.get(location) {
                    Some(x) => x.clone(),
                    None => panic!("No child node"),
                }
            }
        } else if slice == "$ ls" {
            continue; 
        } else if slice == "dir " {
            let name = line.split_whitespace().nth(1).unwrap();
            let option_node = nodes[current_dir].add_child(length, String::from(name));
            let node = match option_node {
                Some(node) => node,
                None => panic!("Node already exists"),
            };
            nodes.push(node);
        } else {
            let vec: Vec<&str> = line.split_whitespace().collect();
            nodes[current_dir].add_file(String::from(vec[1]), vec[0].parse().unwrap());
        }
    }
    return find_delete(nodes);
}



#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let result = run("input_test.txt");
        assert_eq!(result, 24933642)
    }
}