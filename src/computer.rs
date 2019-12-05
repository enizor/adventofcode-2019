use std::convert::TryFrom;
use std::iter::Iterator;
enum OpCode {
    Add([bool; 3]),
    Mul([bool; 3]),
    Sav([bool; 1]),
    Prt([bool; 1]),
    Jnz([bool; 2]),
    Jz([bool; 2]),
    Leq([bool; 3]),
    Eq([bool; 3]),
    End,
    Err,
}

fn parse_modes<const N: usize>(mut value: isize, mut params: [bool; N]) -> [bool; N] {
    let mut mode;
    for param in params.iter_mut() {
        mode = (value) % 10;
        assert!(mode <= 1, "Invalid mode: {}", mode);
        *param = mode == 1;
        value /= 10;
    }
    params
}

impl OpCode {
    fn from_isize(value: isize) -> OpCode {
        if value < 0 {
            Self::Err
        } else {
            match value % 100 {
                1 => Self::Add(parse_modes(value / 100, [false; 3])),
                2 => Self::Mul(parse_modes(value / 100, [false; 3])),
                3 => Self::Sav(parse_modes(value / 100, [false; 1])),
                4 => Self::Prt(parse_modes(value / 100, [false; 1])),
                5 => Self::Jnz(parse_modes(value / 100, [false; 2])),
                6 => Self::Jz(parse_modes(value / 100, [false; 2])),
                7 => Self::Leq(parse_modes(value / 100, [false; 3])),
                8 => Self::Eq(parse_modes(value / 100, [false; 3])),
                99 => Self::End,
                _ => Self::Err,
            }
        }
    }
}

pub struct Computer<I: Iterator<Item = isize>> {
    pub ip: usize,
    pub memory: Vec<isize>,
    pub inputs: I,
    pub outputs: Vec<isize>,
}

impl<I: Iterator<Item = isize>> Computer<I> {
    fn read_with_mode(&mut self, is_immediate: bool) -> Result<isize, &'static str> {
        self.ip += 1;
        if is_immediate {
            Ok(self.memory[self.ip])
        } else {
            let pos = usize::try_from(self.memory[self.ip])
                .map_err(|_| "Reading from a negative pointer!")?;
            Ok(self.memory[pos])
        }
    }

    fn store_with_mode(&mut self, is_immediate: bool, value: isize) -> Result<(), &'static str> {
        self.ip += 1;
        assert!(!is_immediate, "Trying to write while in immediate mode");
        let pos = usize::try_from(self.memory[self.ip])
            .map_err(|_| "Storing into a negative pointer!")?;
        self.memory[pos] = value;
        Ok(())
    }

    fn run_op_code(&mut self) -> Result<bool, &'static str> {
        let op_code = OpCode::from_isize(self.memory[self.ip]);
        let mut jumped = false;
        match op_code {
            OpCode::Add(modes) => {
                let result = self.read_with_mode(modes[0])? + self.read_with_mode(modes[1])?;
                self.store_with_mode(modes[2], result)?
            }
            OpCode::Mul(modes) => {
                let result = self.read_with_mode(modes[0])? * self.read_with_mode(modes[1])?;
                self.store_with_mode(modes[2], result)?
            }
            OpCode::Sav(modes) => {
                let result = self.inputs.next().expect("Need more inputs!");
                self.store_with_mode(modes[0], result)?
            }
            OpCode::Prt(modes) => {
                let result = self.read_with_mode(modes[0])?;
                self.outputs.push(result)
            }
            OpCode::Jnz(modes) => {
                let result = self.read_with_mode(modes[0])?;
                let jump = self.read_with_mode(modes[1])?;
                if result != 0 {
                    self.ip =
                        usize::try_from(jump).map_err(|_| "Jumping into a negative pointer!")?;
                    jumped = true;
                }
            }
            OpCode::Jz(modes) => {
                let result = self.read_with_mode(modes[0])?;
                let jump = self.read_with_mode(modes[1])?;
                if result == 0 {
                    self.ip =
                        usize::try_from(jump).map_err(|_| "Jumping into a negative pointer!")?;
                    jumped = true;
                }
            }
            OpCode::Leq(modes) => {
                let lhs = self.read_with_mode(modes[0])?;
                let rhs = self.read_with_mode(modes[1])?;
                if lhs < rhs {
                    self.store_with_mode(modes[2], 1)?
                } else {
                    self.store_with_mode(modes[2], 0)?
                }
            }
            OpCode::Eq(modes) => {
                let lhs = self.read_with_mode(modes[0])?;
                let rhs = self.read_with_mode(modes[1])?;
                if lhs == rhs {
                    self.store_with_mode(modes[2], 1)?
                } else {
                    self.store_with_mode(modes[2], 0)?
                }
            }
            OpCode::End => return Ok(false),
            OpCode::Err => return Err("Read a wrong opcode"),
        }
        if !jumped {
            self.ip += 1
        }
        Ok(true)
    }

    pub fn run(&mut self) -> Result<isize, &'static str> {
        loop {
            if !self.run_op_code()? {
                if self.outputs.len() > 0 {
                    return Ok(self.outputs[self.outputs.len() - 1]);
                } else {
                    return Ok(0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[isize] = &[
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    #[test]
    fn leq8() {
        let input = INPUT.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input,
            inputs: std::iter::once(7),
            outputs: Vec::new(),
        };
        let value = computer.run().expect("Error while running program");
        assert_eq!(value, 999);
    }

    #[test]
    fn eq8() {
        let input = INPUT.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input,
            inputs: std::iter::once(8),
            outputs: Vec::new(),
        };
        let value = computer.run().expect("Error while running program");
        assert_eq!(value, 1000);
    }

    #[test]
    fn geq8() {
        let input = INPUT.to_owned();
        let mut computer = Computer {
            ip: 0,
            memory: input,
            inputs: std::iter::once(9),
            outputs: Vec::new(),
        };
        let value = computer.run().expect("Error while running program");
        assert_eq!(value, 1001);
    }
}
