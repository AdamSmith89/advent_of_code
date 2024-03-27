use crate::error::AdventError;

#[derive(Debug, PartialEq)]
pub enum OpCode {
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
pub enum Mode {
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
mod tests {
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