use std::convert::TryFrom;
use std::iter::Iterator;
enum OpCode {
    Add([bool; 3]),
    Mul([bool; 3]),
    Sav([bool; 1]),
    Prt([bool; 1]),
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
            OpCode::Prt(modes) => println!("{}", self.read_with_mode(modes[0])?),
            OpCode::End => return Ok(false),
            OpCode::Err => return Err("Read a wrong opcode"),
        }
        self.ip += 1;
        Ok(true)
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        loop {
            if !self.run_op_code()? {
                return Ok(());
            }
        }
    }
}
