use crate::error::AdventError;

pub struct IntCodeComputer {
    code: Vec<u32>,
    ip: usize,
}

impl IntCodeComputer {
    pub fn new(code: Vec<u32>) -> Self {
        Self { code, ip: 0 }
    }

    pub fn run(&mut self) -> Result<(), AdventError> {
        loop {
            let raw_val = self.read_adv(self.ip)?;

            match OpCode::try_from(raw_val)? {
                OpCode::Add => self.add()?,
                OpCode::Mul => self.mul()?,
                OpCode::End => break,
            }
        }

        Ok(())
    }

    pub fn read(&self, idx: usize) -> Result<u32, AdventError> {
        self.code
            .get(idx)
            .cloned()
            .ok_or(AdventError::NotFound(idx.to_string()))
    }

    // Reads value at `idx` and advances the ip
    fn read_adv(&mut self, idx: usize) -> Result<u32, AdventError> {
        let read = self.read(idx);
        self.ip += 1;

        read
    }

    fn set(&mut self, idx: usize, val: u32) -> Result<(), AdventError> {
        if let Some(cur_val) = self.code.get_mut(idx) {
            *cur_val = val;
            Ok(())
        } else {
            Err(AdventError::NotFound(idx.to_string()))
        }
    }

    fn add(&mut self) -> Result<(), AdventError> {
        // Opcode 1 adds together numbers read from two positions and stores the result in a third position.
        // The three integers immediately after the opcode tell you these three positions -
        // the first two indicate the positions from which you should read the input values,
        // and the third indicates the position at which the output should be stored.

        let input_1 = self.read_adv(self.read(self.ip)? as usize)?;
        let input_2 = self.read_adv(self.read(self.ip)? as usize)?;

        let output_idx = self.read_adv(self.ip)? as usize;
        self.set(output_idx, input_1 + input_2)?;

        Ok(())
    }

    fn mul(&mut self) -> Result<(), AdventError> {
        // Opcode 2 multiplies together numbers read from two positions and stores the result in a third position.
        // The three integers immediately after the opcode tell you these three positions -
        // the first two indicate the positions from which you should read the input values,
        // and the third indicates the position at which the output should be stored.

        let input_1 = self.read_adv(self.read(self.ip)? as usize)?;
        let input_2 = self.read_adv(self.read(self.ip)? as usize)?;

        let output_idx = self.read_adv(self.ip)? as usize;
        self.set(output_idx, input_1 * input_2)?;

        Ok(())
    }
}

enum OpCode {
    Add,
    Mul,
    End,
}

impl TryFrom<u32> for OpCode {
    type Error = AdventError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::End),
            _ => Err(AdventError::UnknownPattern(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icc_basic_end() {
        let mut icc = IntCodeComputer::new(vec![99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![99], icc.code);
    }

    #[test]
    fn icc_basic_add() {
        let mut icc = IntCodeComputer::new(vec![1, 0, 0, 0, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![2, 0, 0, 0, 99], icc.code);
    }

    #[test]
    fn icc_basic_mul() {
        let mut icc = IntCodeComputer::new(vec![2, 3, 0, 3, 99]);

        assert!(icc.run().is_ok());
        assert_eq!(vec![2, 3, 0, 6, 99], icc.code);
    }
}
