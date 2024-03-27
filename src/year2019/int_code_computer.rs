use std::ops::{Add, Mul};

use crate::error::AdventError;

pub struct IntCodeComputer {
    code: Vec<i32>,
    ip: usize,
    input_buffer: Option<i32>,
    output_buffer: Vec<i32>,
}

impl IntCodeComputer {
    pub fn load(code: Vec<i32>) -> Self {
        Self {
            code,
            ip: 0,
            input_buffer: None,
            output_buffer: Vec::new(),
        }
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        //println!("{:?}", self.code);
        loop {
            match self.parse_instruction()? {
                OpCode::Add(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} + {param2}");
                    self.binary_op(param1, param2, Add::add)?
                }
                OpCode::Mul(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} x {param2}");
                    self.binary_op(param1, param2, Mul::mul)?
                }
                OpCode::In => self.input()?,
                OpCode::Out(mode) => {
                    let value = self.next_with_mode(mode)?;

                    //println!("  {value}");
                    self.output(value)?
                }
                OpCode::JNZ(mode1, mode2) => {
                    let value = self.next_with_mode(mode1)?;
                    let jmp_addr = self.next_with_mode(mode2)?;

                    //println!("  if {value} != 0 then jmp {jmp_addr}");
                    self.jmp_op(value, jmp_addr as usize, PartialEq::ne)?
                }
                OpCode::JZ(mode1, mode2) => {
                    let value = self.next_with_mode(mode1)?;
                    let jmp_addr = self.next_with_mode(mode2)?;

                    //println!("  if {value} == 0 then jmp {jmp_addr}");
                    self.jmp_op(value, jmp_addr as usize, PartialEq::eq)?
                }
                OpCode::LT(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} < {param2}");
                    self.cmp_op(param1, param2, PartialOrd::lt)?
                }
                OpCode::EQ(mode1, mode2) => {
                    let param1 = self.next_with_mode(mode1)?;
                    let param2 = self.next_with_mode(mode2)?;

                    //println!("  {param1} == {param2}");
                    self.cmp_op(param1, param2, PartialEq::eq)?
                }
                OpCode::End => break,
            }
        }

        Ok(())
    }

    pub fn set_input(&mut self, input: i32) {
        self.input_buffer = Some(input);
    }

    pub fn get_output(&self) -> &Vec<i32> {
        &self.output_buffer
    }

    pub fn read(&self, addr: usize) -> color_eyre::Result<i32> {
        self.code
            .get(addr)
            .cloned()
            .ok_or(AdventError::NotFound(addr.to_string()).into())
    }

    fn write(&mut self, addr: usize, val: i32) -> color_eyre::Result<()> {
        if let Some(cur_val) = self.code.get_mut(addr) {
            //println!("  Writing {val} to {addr}");
            *cur_val = val;
            Ok(())
        } else {
            Err(AdventError::NotFound(addr.to_string()).into())
        }
    }

    fn parse_instruction(&mut self) -> color_eyre::Result<OpCode> {
        //print!("[{}] ", self.ip);

        let raw_val = self.next()?;
        let op = OpCode::try_from(raw_val)?;

        //let upper = self.code.len().min(self.ip + op.num_params());
        //let context = &self.code[self.ip - 1..upper];
        //println!("{op:?} ({context:?})");

        Ok(op)
    }

    // Reads value at instruction pointer then advances it one step
    fn next(&mut self) -> color_eyre::Result<i32> {
        let value = self.read(self.ip);
        self.ip += 1;

        value
    }

    fn next_with_mode(&mut self, mode: Mode) -> color_eyre::Result<i32> {
        let value = self.next()?;
        match mode {
            Mode::Pos => self.read(value as usize),
            Mode::Imm => Ok(value),
        }
    }

    fn binary_op<Op>(&mut self, param1: i32, param2: i32, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(i32, i32) -> i32,
    {
        // Opcode 1 adds together numbers read from two positions and stores the result in a third position.
        // Opcode 2 multiplies together numbers read from two positions and stores the result in a third position.
        // The three integers immediately after the opcode tell you these three positions -
        // the first two indicate the positions from which you should read the input values,
        // and the third indicates the position at which the output should be stored.

        // Parameters that an instruction writes to will always be in Position Mode
        let addr = self.next()? as usize;
        let result = op(param1, param2);
        self.write(addr, result)?;

        Ok(())
    }

    fn jmp_op<Op>(&mut self, value: i32, jmp_addr: usize, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(&i32, &i32) -> bool,
    {
        // Opcode 5 is jump-if-true: if the first parameter is non-zero,
        // Opcode 6 is jump-if-false: if the first parameter is zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.

        if op(&value, &0) {
            self.ip = jmp_addr;
        }

        Ok(())
    }

    fn cmp_op<Op>(&mut self, param1: i32, param2: i32, op: Op) -> color_eyre::Result<()>
    where
        Op: FnOnce(&i32, &i32) -> bool,
    {
        // Opcode 7 is less than: if the first parameter is less than the second parameter,
        // Opcode 8 is equals: if the first parameter is equal to the second parameter
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0.

        // Parameters that an instruction writes to will always be in Position Mode
        let addr = self.next()? as usize;
        if op(&param1, &param2) {
            self.write(addr, 1)?;
        } else {
            self.write(addr, 0)?;
        }

        Ok(())
    }

    fn input(&mut self) -> color_eyre::Result<()> {
        // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
        // For example, the instruction 3,50 would take an input value and store it at address 50.

        let input = self.input_buffer.ok_or(AdventError::NotFound(String::from(
            "No input value provided",
        )))?;

        // Parameters that an instruction writes to will always be in Position Mode
        let addr = self.next()?;
        self.write(addr as usize, input)?;

        Ok(())
    }

    fn output(&mut self, value: i32) -> color_eyre::Result<()> {
        // Opcode 4 outputs the value of its only parameter.
        // For example, the instruction 4,50 would output the value at address 50.

        // Is this the best way to represent output?
        self.output_buffer.push(value);

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add(Mode, Mode),
    Mul(Mode, Mode),
    In,
    Out(Mode),
    JNZ(Mode, Mode),
    JZ(Mode, Mode),
    LT(Mode, Mode),
    EQ(Mode, Mode),
    End,
}

impl OpCode {
    fn num_params(&self) -> usize {
        match self {
            OpCode::Add(_, _) => 3,
            OpCode::Mul(_, _) => 3,
            OpCode::In => 1,
            OpCode::Out(_) => 1,
            OpCode::JNZ(_, _) => 2,
            OpCode::JZ(_, _) => 2,
            OpCode::LT(_, _) => 2,
            OpCode::EQ(_, _) => 3,
            OpCode::End => 0,
        }
    }
}

impl TryFrom<i32> for OpCode {
    type Error = color_eyre::Report;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let code = value % 100; // The 2 rightmost digits represent the OpCode

        // The remaining digits represent parameter modes and can be different per OpCode
        let mut modes = value / 100;

        match code {
            1 => {
                let mode1 = Mode::try_from(modes % 10)?;
                modes /= 10;
                let mode2 = Mode::try_from(modes % 10)?;

                Ok(Self::Add(mode1, mode2))
            }
            2 => {
                let mode1 = Mode::try_from(modes % 10)?;
                modes /= 10;
                let mode2 = Mode::try_from(modes % 10)?;

                Ok(Self::Mul(mode1, mode2))
            }
            3 => Ok(Self::In),
            4 => {
                let mode = Mode::try_from(modes % 10)?;

                Ok(Self::Out(mode))
            }
            5 => {
                let mode1 = Mode::try_from(modes % 10)?;
                modes /= 10;
                let mode2 = Mode::try_from(modes % 10)?;

                Ok(Self::JNZ(mode1, mode2))
            }
            6 => {
                let mode1 = Mode::try_from(modes % 10)?;
                modes /= 10;
                let mode2 = Mode::try_from(modes % 10)?;

                Ok(Self::JZ(mode1, mode2))
            }
            7 => {
                let mode1 = Mode::try_from(modes % 10)?;
                modes /= 10;
                let mode2 = Mode::try_from(modes % 10)?;

                Ok(Self::LT(mode1, mode2))
            }
            8 => {
                let mode1 = Mode::try_from(modes % 10)?;
                modes /= 10;
                let mode2 = Mode::try_from(modes % 10)?;

                Ok(Self::EQ(mode1, mode2))
            }
            99 => Ok(Self::End),
            _ => Err(AdventError::UnknownPattern(value.to_string()).into()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    Pos,
    Imm,
}

impl TryFrom<i32> for Mode {
    type Error = color_eyre::Report;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Imm),
            _ => Err(AdventError::UnknownPattern(value.to_string()).into()),
        }
    }
}

#[cfg(test)]
mod opcode_tests {
    use super::{Mode, OpCode};

    #[test]
    fn opcode_try_from_add() {
        let result = OpCode::try_from(1);
        assert!(result.is_ok());
        assert_eq!(OpCode::Add(Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_mul() {
        let result = OpCode::try_from(2);
        assert!(result.is_ok());
        assert_eq!(OpCode::Mul(Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_in() {
        let result = OpCode::try_from(3);
        assert!(result.is_ok());
        assert_eq!(OpCode::In, result.unwrap());
    }

    #[test]
    fn opcode_try_from_out() {
        let result = OpCode::try_from(4);
        assert!(result.is_ok());
        assert_eq!(OpCode::Out(Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_jnz() {
        let result = OpCode::try_from(5);
        assert!(result.is_ok());
        assert_eq!(OpCode::JNZ(Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_jz() {
        let result = OpCode::try_from(6);
        assert!(result.is_ok());
        assert_eq!(OpCode::JZ(Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_lt() {
        let result = OpCode::try_from(7);
        assert!(result.is_ok());
        assert_eq!(OpCode::LT(Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_eq() {
        let result = OpCode::try_from(8);
        assert!(result.is_ok());
        assert_eq!(OpCode::EQ(Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_end() {
        let result = OpCode::try_from(99);
        assert!(result.is_ok());
        assert_eq!(OpCode::End, result.unwrap());
    }

    #[test]
    fn opcode_try_from_pos_mode() {
        let result = OpCode::try_from(004);
        assert!(result.is_ok());
        assert_eq!(OpCode::Out(Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_imm_mode() {
        let result = OpCode::try_from(104);
        assert!(result.is_ok());
        assert_eq!(OpCode::Out(Mode::Imm), result.unwrap());
    }

    #[test]
    fn opcode_try_from_mixed_mode() {
        let result = OpCode::try_from(1001);
        assert!(result.is_ok());
        assert_eq!(OpCode::Add(Mode::Pos, Mode::Imm), result.unwrap());
    }

    #[test]
    fn opcode_try_from_mixed_mode_leading_zero() {
        let result = OpCode::try_from(0102);
        assert!(result.is_ok());
        assert_eq!(OpCode::Mul(Mode::Imm, Mode::Pos), result.unwrap());
    }
}

#[cfg(test)]
mod icc_tests {
    use super::IntCodeComputer;

    #[test]
    fn icc_end() {
        let mut icc = IntCodeComputer::load(vec![99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![99], icc.code);
    }

    #[test]
    fn icc_add() {
        let mut icc = IntCodeComputer::load(vec![1, 0, 0, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![2, 0, 0, 0, 99], icc.code);
    }

    #[test]
    fn icc_mul() {
        let mut icc = IntCodeComputer::load(vec![2, 3, 0, 3, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![2, 3, 0, 6, 99], icc.code);
    }

    #[test]
    fn icc_in() {
        let mut icc = IntCodeComputer::load(vec![3, 3, 99, 0]);
        icc.set_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 3, 99, 8], icc.code);
    }

    #[test]
    fn icc_out() {
        let mut icc = IntCodeComputer::load(vec![4, 3, 104, 8, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![4, 3, 104, 8, 99], icc.code);
        assert_eq!(&vec![8, 8], icc.get_output());
    }

    #[test]
    fn icc_jnz_true() {
        // 0 is an invalid OpCode, the JNZ should set the ip to [99]
        let mut icc = IntCodeComputer::load(vec![1105, 1, 4, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1105, 1, 4, 0, 99], icc.code);
    }

    #[test]
    fn icc_jnz_false() {
        // 0 is an invalid OpCode, the JNZ should not change the ip
        let mut icc = IntCodeComputer::load(vec![1105, 0, 4, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1105, 0, 4, 99, 0], icc.code);
    }

    #[test]
    fn icc_jz_true() {
        // 0 is an invalid OpCode, the JZ should set the ip to [99]
        let mut icc = IntCodeComputer::load(vec![1106, 0, 4, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1106, 0, 4, 0, 99], icc.code);
    }

    #[test]
    fn icc_jz_false() {
        // 0 is an invalid OpCode, the JZ should not change the ip
        let mut icc = IntCodeComputer::load(vec![1106, 1, 4, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1106, 1, 4, 99, 0], icc.code);
    }

    #[test]
    fn icc_lt_true() {
        // Set [6] to 1 as [5](0) < 1
        let mut icc = IntCodeComputer::load(vec![1007, 5, 1, 6, 99, 0, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1007, 5, 1, 6, 99, 0, 1], icc.code);
    }

    #[test]
    fn icc_lt_false() {
        // Set [6] to 0 as [5](1) > 0
        let mut icc = IntCodeComputer::load(vec![1007, 5, 0, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1007, 5, 0, 6, 99, 1, 0], icc.code);
    }

    #[test]
    fn icc_lt_eq() {
        // Set [6] to 0 as [5](1) == 1
        let mut icc = IntCodeComputer::load(vec![1007, 5, 1, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1007, 5, 1, 6, 99, 1, 0], icc.code);
    }

    #[test]
    fn icc_eq_true() {
        // Set [6] to 1 as [5](1) == 1
        let mut icc = IntCodeComputer::load(vec![1008, 5, 1, 6, 99, 1, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1008, 5, 1, 6, 99, 1, 1], icc.code);
    }

    #[test]
    fn icc_eq_false() {
        // Set [6] to 0 as [5](1) != 0
        let mut icc = IntCodeComputer::load(vec![1008, 5, 0, 6, 99, 1, 1]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1008, 5, 0, 6, 99, 1, 0], icc.code);
    }

    #[test]
    fn icc_add_neg() {
        let mut icc = IntCodeComputer::load(vec![1101, -4, 3, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![-1, -4, 3, 0, 99], icc.code);
    }

    #[test]
    fn icc_add_imm() {
        let mut icc = IntCodeComputer::load(vec![1101, 2, 4, 5, 99, 0]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![1101, 2, 4, 5, 99, 6], icc.code);
    }

    #[test]
    fn icc_day05_part2_example1() {
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        icc.set_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8], icc.code);
        assert_eq!(&vec![1], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example2() {
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        icc.set_input(10);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], icc.code);
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example3() {
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        icc.set_input(10);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 3, 1108, 0, 8, 3, 4, 3, 99], icc.code);
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example4() {
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).
        let mut icc = IntCodeComputer::load(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        icc.set_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(vec![3, 3, 1107, 1, 8, 3, 4, 3, 99], icc.code);
        assert_eq!(&vec![1], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example5() {
        // Using position mode, take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero
        let mut icc = IntCodeComputer::load(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        icc.set_input(0);

        assert!(icc.run().is_ok());
        assert_eq!(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 0, 0, 1, 9],
            icc.code
        );
        assert_eq!(&vec![0], icc.get_output());
    }

    #[test]
    fn icc_day05_part2_example6() {
        // Using immediate mode, take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero
        let mut icc = IntCodeComputer::load(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        icc.set_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(
            vec![3, 3, 1105, 5, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            icc.code
        );
        assert_eq!(&vec![1], icc.get_output());
    }

    // year2019::day05 part 2 big example
    // The example program uses an input instruction to ask for a single number.
    // The program will then output 999 if the input value is below 8,
    // ...
    #[test]
    fn icc_day05_part2_big_example_lt() {
        let mut icc = IntCodeComputer::load(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        icc.set_input(5);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![999], icc.get_output());
    }

    // ...
    // output 1000 if the input value is equal to 8,
    // ...
    #[test]
    fn icc_day05_part2_big_example_eq() {
        let mut icc = IntCodeComputer::load(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        icc.set_input(8);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![1000], icc.get_output());
    }

    // ...
    // or output 1001 if the input value is greater than 8.
    #[test]
    fn icc_day05_part2_big_example_gt() {
        let mut icc = IntCodeComputer::load(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        icc.set_input(15);

        assert!(icc.run().is_ok());
        assert_eq!(&vec![1001], icc.get_output());
    }
}
