use crate::read_file_str;

struct Register {
    data: i32,
    cycles: i32,
}

impl Register {
    fn cycle(&mut self) -> bool {
        self.cycles += 1;
        if self.cycles == 40{
            self.cycles = 0;
            return true;
        }
        return false;
    }

    fn addx(&mut self, num: i32) {
        self.data += num;
    }

    fn get_char(&self) -> char {
        if self.cycles >= self.data -1 && self.cycles <= self.data + 1 {
            return '#';
        }
        return '.';
    }
}

pub fn run(input: &str) -> usize {
    let input_data = read_file_str(input);
    let mut reg = Register {data: 1, cycles: 0};
    let mut data = String::new();
    for line in input_data.lines() {
        let mut line_split = line.split_whitespace();
        let arg_1 = line_split.next().unwrap();
        data.push(reg.get_char());
        if reg.cycle() {
            println!("{}", data);
            data.clear();
        }
        if arg_1 == "addx" {
            data.push(reg.get_char());
            let arg_2 = line_split
                .next().unwrap()
                .parse::<i32>().unwrap();
            if reg.cycle() {
                println!("{}", data);
                data.clear();
            }
            reg.addx(arg_2);
        }
    }
    return 0;
}

