use crate::util::computer::{self, Computer};
use itertools::Itertools;

#[aoc_generator(day07)]
pub fn day07_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[inline]
fn run_amplifier(code: &[i64], phase_setting: i64, input_signal: i64) -> computer::Result<i64> {
    Computer::new(code, None).resume_get([phase_setting, input_signal].iter().copied())
}

fn get_signal(code: &[i64], inputs: &[i64]) -> computer::Result<i64> {
    let mut signal = run_amplifier(code, inputs[0], 0)?;
    signal = run_amplifier(code, inputs[1], signal)?;
    signal = run_amplifier(code, inputs[2], signal)?;
    signal = run_amplifier(code, inputs[3], signal)?;
    run_amplifier(code, inputs[4], signal)
}

#[aoc(day07, part1)]
pub fn day07_part1(input: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|i| get_signal(input, &i).unwrap())
        .sorted()
        .last()
        .unwrap()
}

fn get_signal_feedback(code: &[i64], inputs: &[i64]) -> computer::Result<i64> {
    let mut computers: Vec<Computer> = Vec::new();
    let mut signal = 0;
    for i in (0..inputs.len()).cycle() {
        match computers.get_mut(i) {
            Some(computer) => match computer.resume(Some(signal))? {
                Some(s) => signal = s,
                None => return Ok(signal),
            },
            None => {
                let mut computer = Computer::new(code, None);
                match computer.resume([inputs[i], signal].iter().copied())? {
                    Some(s) => signal = s,
                    None => return Ok(signal),
                }
                computers.push(computer)
            }
        }
    }
    unreachable!()
}

#[aoc(day07, part2)]
pub fn day07_part2(input: &[i64]) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|i| get_signal_feedback(input, &i).unwrap())
        .sorted()
        .last()
        .unwrap()
}
