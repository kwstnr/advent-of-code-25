#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
    R(u16),
    L(u16)
}

pub fn parse(lines: Vec<&str>) -> Vec<Instruction> {
    lines.iter()
        .map(|line| parse_line(line))        
        .collect()
}

pub fn parse_line(line: &str) -> Instruction {
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
}
