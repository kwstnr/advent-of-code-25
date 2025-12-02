use crate::parser::Instruction;

pub struct Dial {
    pub position: u16,
    pub zero_counter: u16
}

impl Dial {
    pub fn new(starting_position: u8) -> Self {
        Dial {
            position: <u16>::from(starting_position),
            zero_counter: 0
        }
    }

    fn rotate_left(position: u16, amount: u16) -> u16 {
        let amount = amount.rem_euclid(100);
        if position < amount {
            100 - (amount - position)
        } else {
            position - amount
        }
    }

    pub fn rotate(self, instruction: &Instruction) -> Self {
        let new_position = match instruction {
            Instruction::R(amount) => (self.position + amount).rem_euclid(100),
            Instruction::L(amount) => Dial::rotate_left(self.position, *amount)
        };
        Dial {
            position: new_position,
            zero_counter: self.zero_counter + <bool as Into<u16>>::into(new_position == 0)
        }
    }

    pub fn rotate_part2(self, instruction: &Instruction) -> Self {
        let (new_position, times_surpassed_zero) = match instruction {
            Instruction::R(amount) => (
                (self.position + amount).rem_euclid(100),
                (self.position + amount) / 100
            ),
            Instruction::L(amount) => (
                Dial::rotate_left(self.position, *amount),
                (amount / 100) + <bool as Into<u16>>::into(self.position != 0 && amount.rem_euclid(100) >= self.position)
            )
        };
        Dial {
            position: new_position,
            zero_counter: self.zero_counter + times_surpassed_zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right_1() {
        let dial = Dial {
            position: 40,
            zero_counter: 0
        };
        let instruction = Instruction::R(60);
        let expected_position = 0;
        let expected_zero_counter = 1;

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

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

        let result = dial.rotate(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_left_gt100_1() {
        let dial = Dial {
            position: 90,
            zero_counter: 0
        };
        let instruction = Instruction::L(290);
        let expected_position = 0;
        let expected_zero_counter = 3;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_right_gt100_1() {
        let dial = Dial {
            position: 90,
            zero_counter: 0
        };
        let instruction = Instruction::R(290);
        let expected_position = 80;
        let expected_zero_counter = 3;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_right_gt100_2() {
        let dial = Dial {
            position: 90,
            zero_counter: 0
        };
        let instruction = Instruction::R(210);
        let expected_position = 0;
        let expected_zero_counter = 3;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_right_gt100_3() {
        let dial = Dial {
            position: 90,
            zero_counter: 0
        };
        let instruction = Instruction::R(209);
        let expected_position = 99;
        let expected_zero_counter = 2;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_right_1() {
        let dial = Dial {
            position: 88,
            zero_counter: 6
        };
        let instruction = Instruction::R(20);
        let expected_position = 8;
        let expected_zero_counter = 7;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_right_2() {
        let dial = Dial {
            position: 88,
            zero_counter: 0
        };
        let instruction = Instruction::R(12);
        let expected_position = 0;
        let expected_zero_counter = 1;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_right_3() {
        let dial = Dial {
            position: 88,
            zero_counter: 0
        };
        let instruction = Instruction::R(11);
        let expected_position = 99;
        let expected_zero_counter = 0;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_left_1() {
        let dial = Dial {
            position: 88,
            zero_counter: 6
        };
        let instruction = Instruction::L(89);
        let expected_position = 99;
        let expected_zero_counter = 7;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_left_2() {
        let dial = Dial {
            position: 90,
            zero_counter: 0
        };
        let instruction = Instruction::L(90);
        let expected_position = 0;
        let expected_zero_counter = 1;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_left_3() {
        let dial = Dial {
            position: 90,
            zero_counter: 0
        };
        let instruction = Instruction::L(89);
        let expected_position = 1;
        let expected_zero_counter = 0;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }

    #[test]
    fn rotate_part2_left_4() {
        let dial = Dial {
            position: 0,
            zero_counter: 0
        };
        let instruction = Instruction::L(99);
        let expected_position = 1;
        let expected_zero_counter = 0;

        let result = dial.rotate_part2(&instruction);

        assert_eq!(expected_position, result.position);
        assert_eq!(expected_zero_counter, result.zero_counter);
    }
}
