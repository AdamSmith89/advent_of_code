use crate::error::AdventError;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    In(Mode),
    Out(Mode),
    JNZ(Mode, Mode),
    JZ(Mode, Mode),
    LT(Mode, Mode, Mode),
    EQ(Mode, Mode, Mode),
    RBO(Mode),
    End,
}

impl OpCode {
    pub fn num_params(&self) -> usize {
        match self {
            OpCode::Add(_, _, _) => 3,
            OpCode::Mul(_, _, _) => 3,
            OpCode::In(_) => 1,
            OpCode::Out(_) => 1,
            OpCode::JNZ(_, _) => 2,
            OpCode::JZ(_, _) => 2,
            OpCode::LT(_, _, _) => 3,
            OpCode::EQ(_, _, _) => 3,
            OpCode::RBO(_) => 1,
            OpCode::End => 0,
        }
    }
}

impl TryFrom<i64> for OpCode {
    type Error = color_eyre::Report;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let code = value % 100; // The 2 rightmost digits represent the OpCode

        // The remaining digits represent parameter modes and can be different per OpCode
        let mut modes = value / 100;

        let get_next_mode = |modes: &mut i64, fail_on_imm: bool| -> color_eyre::Result<Mode> {
            let mode = Mode::try_from(*modes % 10)?;

            if mode == Mode::Imm && fail_on_imm {
                return Err(AdventError::LogicError(String::from(
                    "Parameters an instruction writes to should never be in immediate mode",
                ))
                .into());
            }

            *modes /= 10;
            Ok(mode)
        };

        match code {
            1 => {
                let mode1 = get_next_mode(&mut modes, false)?;
                let mode2 = get_next_mode(&mut modes, false)?;
                let mode3 = get_next_mode(&mut modes, true)?;

                Ok(Self::Add(mode1, mode2, mode3))
            }
            2 => {
                let mode1 = get_next_mode(&mut modes, false)?;
                let mode2 = get_next_mode(&mut modes, false)?;
                let mode3 = get_next_mode(&mut modes, true)?;

                Ok(Self::Mul(mode1, mode2, mode3))
            }
            3 => {
                let mode = get_next_mode(&mut modes, true)?;

                Ok(Self::In(mode))
            }
            4 => {
                let mode = get_next_mode(&mut modes, false)?;

                Ok(Self::Out(mode))
            }
            5 => {
                let mode1 = get_next_mode(&mut modes, false)?;
                let mode2 = get_next_mode(&mut modes, false)?;

                Ok(Self::JNZ(mode1, mode2))
            }
            6 => {
                let mode1 = get_next_mode(&mut modes, false)?;
                let mode2 = get_next_mode(&mut modes, false)?;

                Ok(Self::JZ(mode1, mode2))
            }
            7 => {
                let mode1 = get_next_mode(&mut modes, false)?;
                let mode2 = get_next_mode(&mut modes, false)?;
                let mode3 = get_next_mode(&mut modes, true)?;

                Ok(Self::LT(mode1, mode2, mode3))
            }
            8 => {
                let mode1 = get_next_mode(&mut modes, false)?;
                let mode2 = get_next_mode(&mut modes, false)?;
                let mode3 = get_next_mode(&mut modes, true)?;

                Ok(Self::EQ(mode1, mode2, mode3))
            }
            9 => {
                let mode = get_next_mode(&mut modes, false)?;

                Ok(Self::RBO(mode))
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
    Rel,
}

impl TryFrom<i64> for Mode {
    type Error = color_eyre::Report;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Imm),
            2 => Ok(Mode::Rel),
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
        assert_eq!(
            OpCode::Add(Mode::Pos, Mode::Pos, Mode::Pos),
            result.unwrap()
        );
    }

    #[test]
    fn opcode_try_from_add_imm_err() {
        let result = OpCode::try_from(10001);
        assert!(result.is_err());
    }

    #[test]
    fn opcode_try_from_mul() {
        let result = OpCode::try_from(2);
        assert!(result.is_ok());
        assert_eq!(
            OpCode::Mul(Mode::Pos, Mode::Pos, Mode::Pos),
            result.unwrap()
        );
    }

    #[test]
    fn opcode_try_from_in() {
        let result = OpCode::try_from(3);
        assert!(result.is_ok());
        assert_eq!(OpCode::In(Mode::Pos), result.unwrap());
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
        assert_eq!(OpCode::LT(Mode::Pos, Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_eq() {
        let result = OpCode::try_from(8);
        assert!(result.is_ok());
        assert_eq!(OpCode::EQ(Mode::Pos, Mode::Pos, Mode::Pos), result.unwrap());
    }

    #[test]
    fn opcode_try_from_arb() {
        let result = OpCode::try_from(9);
        assert!(result.is_ok());
        assert_eq!(OpCode::RBO(Mode::Pos), result.unwrap());
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
        let result = OpCode::try_from(21001);
        assert!(result.is_ok());
        assert_eq!(
            OpCode::Add(Mode::Pos, Mode::Imm, Mode::Rel),
            result.unwrap()
        );
    }

    #[test]
    fn opcode_try_from_mixed_mode_leading_zero() {
        let result = OpCode::try_from(02102);
        assert!(result.is_ok());
        assert_eq!(
            OpCode::Mul(Mode::Imm, Mode::Rel, Mode::Pos),
            result.unwrap()
        );
    }

    #[test]
    fn opcode_try_from_rel_mode() {
        let result = OpCode::try_from(22201);
        assert!(result.is_ok());
        assert_eq!(
            OpCode::Add(Mode::Rel, Mode::Rel, Mode::Rel),
            result.unwrap()
        );
    }
}
