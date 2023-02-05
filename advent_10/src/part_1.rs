use crate::read_file_str;

struct Register {
    data: i32,
    cycles: i32,
}

impl Register {
    fn cycle(&mut self) -> bool {
        self.cycles += 1;
        if (self.cycles-20) % 40 == 0{
            return true;
        }
        return false;
    }

    fn addx(&mut self, num: i32) {
        self.data += num;
    }

    fn calc_signal(&mut self) -> i32 {
        self.data * self.cycles
    }
}

pub fn run(input: &str) -> i32 {
    let data = read_file_str(input);
    let mut reg = Register {data: 1, cycles: 0};
    let mut sum = 0;
    for line in data.lines() {
        let mut line_split = line.split_whitespace();
        let arg_1 = line_split.next().unwrap();
        if reg.cycle() {
            let strength = reg.calc_signal();
            sum += strength;
        }
        if arg_1 == "addx" {
            let arg_2 = line_split
                .next().unwrap()
                .parse::<i32>().unwrap();
            if reg.cycle() {
                let strength = reg.calc_signal();
                sum += strength;
            }
            reg.addx(arg_2);
        }
    }
    return sum;
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let result = run("input_test.txt");
        assert_eq!(result, 13140)
    }
}