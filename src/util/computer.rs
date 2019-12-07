use std::{error, fmt};

pub struct Computer {
    mem: Vec<i32>,
    ip: i32,
    stopped: bool,
}

impl Computer {
    pub fn new(code: &[i32]) -> Computer {
        Computer {
            mem: Vec::from(code),
            ip: 0,
            stopped: false,
        }
    }

    #[inline]
    fn error(&self, kind: ErrorKind) -> Error {
        Error {
            location: self.ip,
            kind,
        }
    }

    #[inline]
    pub fn read_raw(&self, index: i32) -> Result<i32> {
        self.mem
            .get(index as usize)
            .copied()
            .ok_or_else(|| self.error(ErrorKind::InvalidRead(index)))
    }

    #[inline]
    pub fn write_raw(&mut self, index: i32, value: i32) -> Result<()> {
        if (index as usize) < self.mem.len() {
            self.mem[index as usize] = value;
            Ok(())
        } else {
            Err(self.error(ErrorKind::InvalidWrite(index, value)))
        }
    }

    #[inline]
    fn read(&self, mode: Mode, index: i32) -> Result<i32> {
        let param = self.read_raw(index)?;
        match mode {
            Mode::Immediate => Ok(param),
            Mode::Position => self.read_raw(param),
        }
    }

    #[inline]
    fn write(&mut self, index: i32, value: i32) -> Result<()> {
        let addr = self.read_raw(index)?;
        self.write_raw(addr, value)
    }

    #[inline]
    fn input(&mut self, input: Option<i32>) -> Result<i32> {
        input.ok_or(Error {
            location: self.ip,
            kind: ErrorKind::NoInput,
        })
    }

    #[inline]
    fn decode_instruction(&self, index: i32, modes: &mut [Mode; 3]) -> i32 {
        let insn = self.mem[index as usize];
        modes[0] = Mode::from_code(insn / 100 % 10);
        modes[1] = Mode::from_code(insn / 1000 % 10);
        modes[2] = Mode::from_code(insn / 10000 % 10);
        insn % 100
    }

    #[inline]
    fn run_instruction(&mut self, input: Option<i32>) -> Result<Action> {
        let mut modes = [Mode::Position; 3];
        let ip = self.ip;
        match self.decode_instruction(ip, &mut modes) {
            1 => {
                // add: p3 = p1 + p2
                self.write(
                    ip + 3,
                    self.read(modes[0], ip + 1)? + self.read(modes[1], ip + 2)?,
                )?;
                self.ip += 4;
            }
            2 => {
                // mul: p3 = p1 * p2
                self.write(
                    ip + 3,
                    self.read(modes[0], ip + 1)? * self.read(modes[1], ip + 2)?,
                )?;
                self.ip += 4;
            }
            3 => {
                // ipt: p1 = <input>
                let i = self.input(input)?;
                self.write(ip + 1, i)?;
                self.ip += 2;
                return Ok(Action::Input);
            }
            4 => {
                // out: p1 -> <output>
                self.ip += 2;
                return Ok(Action::Output(self.read(modes[0], ip + 1)?));
            }
            5 => {
                // jnz: if p1 != 0 { ip = p2 }
                self.ip = if self.read(modes[0], ip + 1)? != 0 {
                    self.read(modes[1], ip + 2)?
                } else {
                    ip + 3
                }
            }
            6 => {
                // jpz: if p1 == 0 { ip = p2 }
                self.ip = if self.read(modes[0], ip + 1)? == 0 {
                    self.read(modes[1], ip + 2)?
                } else {
                    ip + 3
                }
            }
            7 => {
                // clt: p3 = p1 < p2 ? 1 : 0
                self.write(
                    ip + 3,
                    (self.read(modes[0], ip + 1)? < self.read(modes[1], ip + 2)?) as i32,
                )?;
                self.ip += 4
            }
            8 => {
                // ceq: p3 = p1 == p2 ? 1 : 0
                self.write(
                    ip + 3,
                    (self.read(modes[0], ip + 1)? == self.read(modes[1], ip + 2)?) as i32,
                )?;
                self.ip += 4
            }
            99 => return Ok(Action::Shutdown),
            insn => return Err(self.error(ErrorKind::IllegalOpcode(insn))),
        };
        Ok(Action::Continue)
    }

    pub fn resume<I>(&mut self, inputs: I) -> Result<Option<i32>>
    where
        I: IntoIterator<Item = i32>,
    {
        if self.stopped {
            return Ok(None);
        }

        let mut iter = inputs.into_iter();
        let mut input = iter.next();
        loop {
            match self.run_instruction(input)? {
                Action::Input => input = iter.next(),
                Action::Output(out) => return Ok(Some(out)),
                Action::Shutdown => {
                    self.stopped = true;
                    return Ok(None);
                }
                Action::Continue => (),
            }
        }
    }

    #[inline]
    pub fn resume_get<I>(&mut self, inputs: I) -> Result<i32>
    where
        I: IntoIterator<Item = i32>,
    {
        match self.resume(inputs) {
            Ok(None) => Err(self.error(ErrorKind::NoOutput)),
            Ok(Some(o)) => Ok(o),
            Err(err) => Err(err),
        }
    }

    #[inline]
    pub fn resume_iter<I>(self, inputs: I) -> ResumeIter<I>
    where
        I: IntoIterator<Item = i32>,
    {
        ResumeIter {
            computer: self,
            finished: false,
            inputs,
        }
    }
}

#[derive(Debug)]
enum Action {
    Shutdown,
    Continue,
    Input,
    Output(i32),
}

#[derive(Debug)]
pub struct Error {
    location: i32,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IllegalOpcode(i32),
    InvalidRead(i32),
    InvalidWrite(i32, i32),
    NoInput,
    NoOutput,
}

pub type Result<T> = std::result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::IllegalOpcode(_) => "illegal opcode",
            ErrorKind::NoInput => "reached read instruction with no user input",
            ErrorKind::NoOutput => "program did not return a value",
            ErrorKind::InvalidRead(_) => "tried to read value outside memory bounds",
            ErrorKind::InvalidWrite(_, _) => "tried to write value outside memory bounds",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IntCode error at {}: {}",
            self.location,
            error::Error::description(self)
        )?;
        match self.kind {
            ErrorKind::IllegalOpcode(op) => write!(f, " {}", op),
            ErrorKind::InvalidRead(addr) => write!(f, " at address {}", addr),
            ErrorKind::InvalidWrite(addr, val) => {
                write!(f, ", write value {} at address {}", val, addr)
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Immediate,
    Position,
}

impl Mode {
    fn from_code(code: i32) -> Mode {
        match code {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Invalid parameter mode: {}", code),
        }
    }
}

pub struct ResumeIter<I> {
    inputs: I,
    computer: Computer,
    finished: bool,
}

impl<I> Iterator for ResumeIter<I>
where
    I: Iterator<Item = i32>,
{
    type Item = Result<i32>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match self.computer.resume(&mut self.inputs) {
            Ok(Some(out)) => Some(Ok(out)),
            Ok(None) => {
                self.finished = true;
                None
            }
            Err(err) => {
                self.finished = true;
                Some(Err(err))
            }
        }
    }
}
