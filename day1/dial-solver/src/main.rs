use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read the file");
    let lines = file_content.lines().collect::<Vec<&str>>();
    let instructions = parse(lines);

    let result = instructions
        .iter()
        .fold(Dial::new(), |acc, instruction| acc.rotate(instruction.clone()));

    println!("result: {0}", result.zero_counter);
}

#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    R(u16),
    L(u16)
}

pub struct Dial {
    pub position: u16,
    pub zero_counter: u16
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            position: 0,
            zero_counter: 0
        }
    }

    fn rotate_left(position: u16, amount: u16) -> u16 {
        let amount = amount % 100;
        if position < amount {
            100 - (amount - position)
        } else {
            position - amount
        }
    }

    pub fn rotate(self, instruction: Instruction) -> Self {
        println!("current zero_counter: {0}", self.zero_counter);
        let new_position = match instruction {
                    Instruction::L(amount) => Dial::rotate_left(self.position, amount),
                    Instruction::R(amount) => (self.position + amount) % 100
                };
        if new_position == 0 {
            println!("new_position: {0}", new_position);
        }
        
        Dial {
            position: new_position,
            zero_counter: if new_position == 0 { 
                self.zero_counter + 1
            } else { 
                self.zero_counter
            }
        }
    }
}

pub fn parse(lines: Vec<&str>) -> Vec<Instruction> {
    lines.iter()
        .map(|line| parse_line(line))        
        .collect()
}

pub fn parse_line(line: &str) -> Instruction {
    println!("parsing line: {line}");
    let instr = line.chars().next().unwrap();
    let num: u16 = line[1..].parse().unwrap();
    match instr {
        'L' => Instruction::L(num),
        'R' => Instruction::R(num),
        _ => panic!("not readable instruction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_parse_line_left_sub100() {
        let input = "L35";
        let expected = Instruction::L(35);

        let result = parse_line(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_line_right_sub100() {
        let input = "R35";
        let expected = Instruction::R(35);

        let result = parse_line(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_line_right_gt100() {
        let input = "R435";
        let expected = Instruction::R(435);

        let result = parse_line(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_line_left_gt100() {
        let input = "L435";
        let expected = Instruction::L(435);

        let result = parse_line(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn rotate_right_1() {
        let dial = Dial {
            position: 40,
            zero_counter: 0
        };
        let instruction = Instruction::R(60);
        let expected_position = 0;
        let expected_zero_counter = 1;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_right_2() {
        let dial = Dial {
            position: 40,
            zero_counter: 0
        };
        let instruction = Instruction::R(59);
        let expected_position = 99;
        let expected_zero_counter = 0;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_right_gt100_1() {
        let dial = Dial {
            position: 40,
            zero_counter: 0
        };
        let instruction = Instruction::R(400);
        let expected_position = 40;
        let expected_zero_counter = 0;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_right_gt100_2() {
        let dial = Dial {
            position: 40,
            zero_counter: 6
        };
        let instruction = Instruction::R(460);
        let expected_position = 0;
        let expected_zero_counter = 7;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_left_1() {
        let dial = Dial {
            position: 40,
            zero_counter: 0
        };
        let instruction = Instruction::L(39);
        let expected_position = 1;
        let expected_zero_counter = 0;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_left_2() {
        let dial = Dial {
            position: 40,
            zero_counter: 0
        };
        let instruction = Instruction::L(41);
        let expected_position = 99;
        let expected_zero_counter = 0;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_left_3() {
        let dial = Dial {
            position: 88,
            zero_counter: 6
        };
        let instruction = Instruction::L(88);
        let expected_position = 0;
        let expected_zero_counter = 7;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_left_gt100_1() {
        let dial = Dial {
            position: 88,
            zero_counter: 6
        };
        let instruction = Instruction::L(388);
        let expected_position = 0;
        let expected_zero_counter = 7;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_left_gt100_2() {
        let dial = Dial {
            position: 88,
            zero_counter: 6
        };
        let instruction = Instruction::L(398);
        let expected_position = 90;
        let expected_zero_counter = 6;

        let result = dial.rotate(instruction.clone());

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    //#[test]
    //fn zero_counter_increase() {
    //    let mut n = 0;
    //    while n < 100 {
    //        let random_position: u16 = rand::thread_rng().gen();
    //        let random_amount: u16 = rand::thread_rng().gen();
    //        let left_or_right = rand::thread_rng().gen_range(0..1);
    //        let instruction = {
    //            if (let_or_right == 0) {
    //                Instruction::L(random_amount)
    //            } else {
    //                Instruction::R(random_amount)
    //            }
    //        }

    //        let should_increase_zero_counter = {
    //            match instruction {
    //                Instruction::L(_) => random_position - (random_amount % 100) == 0,
    //                Instruction::R(_) => (random_position + random_amount) % 100 == 0
    //            }
    //        };
    //        n = n + 1;
    //    }

    //}
}
