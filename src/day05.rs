use regex::Regex;
use std::io::{self, prelude::*};

#[aoc_generator(day05)]
pub fn day05_gen(input: &str) -> Vec<i32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

struct Computer {
    mem: Vec<i32>,
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
    fn read(self, computer: &Computer, param: i32) -> i32 {
        match self {
            Mode::Immediate => param,
            Mode::Position => computer.mem[param as usize],
        }
    }
}

impl Computer {
    pub fn new(input: &[i32]) -> Computer {
        Computer {
            mem: Vec::from(input),
        }
    }

    #[inline]
    fn write(&mut self, index: i32, value: i32) {
        let addr = self.mem[index as usize];
        println!("write {} to address {}", value, addr);
        self.mem[addr as usize] = value;
    }

    //#[inline]
    //fn write_indirect(&mut self, mode: Mode, param: i32, value: i32) {
    //    let addr = self.read(mode, param);
    //    println!("iwrite ({:?}): {} -> {} = {}", mode, param, addr, value);
    //    self.mem[addr as usize] = value;
    //}

    #[inline]
    fn read(&self, mode: Mode, index: i32) -> i32 {
        //println!(
        //    "read ({:?}): mem[{}] == {} -> {}",
        //    mode,
        //    index,
        //    self.mem[index as usize],
        //    mode.read(self, self.mem[index as usize])
        //);
        mode.read(self, self.mem[index as usize])
    }

    #[inline]
    fn input(&mut self) -> i32 {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"-?\d+").unwrap();
        }
        let mut buf = String::new();
        print!("< ");
        io::stdout().lock().flush().unwrap();
        io::stdin().lock().read_line(&mut buf).unwrap();
        //println!(
        //    "i: {}",
        //    PATTERN.find(&buf).unwrap().as_str().parse::<i32>().unwrap()
        //);
        PATTERN.find(&buf).unwrap().as_str().parse().unwrap()
    }

    #[inline]
    fn output(&mut self, n: i32) {
        println!("> {}", n);
    }

    #[inline]
    fn decode_instruction(&self, index: i32, modes: &mut [Mode; 3]) -> i32 {
        let insn = self.mem[index as usize];
        modes[0] = Mode::from_code(insn / 100 % 10);
        modes[1] = Mode::from_code(insn / 1000 % 10);
        modes[2] = Mode::from_code(insn / 10000 % 10);
        println!("instruction: {}, modes: {:?}", insn % 100, modes);
        insn % 100
    }

    #[inline]
    fn step(&mut self, i: i32) -> Option<i32> {
        let mut modes = [Mode::Position; 3];
        println!("@ {}: ", i);
        match self.decode_instruction(i, &mut modes) {
            1 => {
                // add: p3 = p1 + p2
                self.write(
                    i + 3,
                    self.read(modes[0], i + 1) + self.read(modes[1], i + 2),
                );
                Some(i + 4)
            }
            2 => {
                // mul: p3 = p1 * p2
                self.write(
                    i + 3,
                    self.read(modes[0], i + 1) * self.read(modes[1], i + 2),
                );
                Some(i + 4)
            }
            3 => {
                // ipt: p1 = <stdin>
                let input = self.input();
                self.write(i + 1, input);
                Some(i + 2)
            }
            4 => {
                // out: p1 -> <stdout>
                self.output(self.read(modes[0], i + 1));
                Some(i + 2)
            }
            5 => {
                // jnz: if p1 != 0 { ip = p2 }
                if self.read(modes[0], i + 1) != 0 {
                    Some(self.read(modes[1], i + 2))
                } else {
                    Some(i + 3)
                }
            }
            6 => {
                // jpz: if p1 == 0 { ip = p2 }
                if self.read(modes[0], i + 1) == 0 {
                    Some(self.read(modes[1], i + 2))
                } else {
                    Some(i + 3)
                }
            }
            7 => {
                // clt: p3 = p1 < p2 ? 1 : 0
                println!(
                    "{} < {}? {}",
                    self.read(modes[0], i + 1),
                    self.read(modes[1], i + 2),
                    self.read(modes[0], i + 1) < self.read(modes[1], i + 2)
                );
                self.write(
                    i + 3,
                    (self.read(modes[0], i + 1) < self.read(modes[1], i + 2)) as i32,
                );
                Some(i + 4)
            }
            8 => {
                // ceq: p3 = p1 == p2 ? 1 : 0
                self.write(
                    i + 3,
                    (self.read(modes[0], i + 1) == self.read(modes[1], i + 2)) as i32,
                );
                Some(i + 4)
            }
            99 => None,
            insn => panic!("Illegal instruction '{}' at postition {}", insn, i),
        }
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while let Some(next) = self.step(i) {
            if next as usize >= self.mem.len() {
                break;
            }
            i = next;
        }
    }
}

#[aoc(day05, part1)]
pub fn day05_part1(_input: &[i32]) -> u32 {
    let input = &[
        3, 21, // get user input, store to $21
        1008, 21, 8, 20, // $20 = input == 8
        1005, 20, 22, // if ($20 != 0) { jump to $22 } (input equal to 8)
        107, 8, 21, 20, // $20 = 8 < input
        1006, 20, 31, // if ($20 == 0) { jump to $31 } (input greater than 8)
        1106, 0, 36, // jump to $36
        98, // (unreachable)
        0,  // $20: used as a variable
        0,  // $21: used as a variable
        1002, 21, 125, 20, // $22: multiply $21 by 125, store the result to $20
        4, 20, // print $20. ($21 * 125)
        1105, 1, 46, // jump to $46 (end)
        104, 999, // print 999
        1105, 1, 46, // jump to $46 (end)
        1101, 1000, 1, 20, // $20 == 1000 + 1
        4, 20, // print $20 (which is 1001)
        1105, 1, 46, // jump to $46 (end)
        98, // (unused)
        99, // end
    ];
    Computer::new(input).run();
    0
}
