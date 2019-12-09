use crate::util::computer::{self, Computer};

#[aoc_generator(day02)]
pub fn day02_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn run_params(computer: &mut Computer, noun: i64, verb: i64) -> computer::Result<i64> {
    computer.write_raw(1, noun)?;
    computer.write_raw(2, verb)?;
    computer.resume(None)?;
    computer.read_raw(0)
}

#[aoc(day02, part1)]
pub fn day02_part1(input: &[i64]) -> i64 {
    const NOUN: i64 = 12;
    const VERB: i64 = 2;

    run_params(&mut Computer::new(input, None), NOUN, VERB).unwrap()
}

#[aoc(day02, part2)]
pub fn day02_part2(input: &[i64]) -> i64 {
    const OUTPUT: i64 = 19_690_720;

    (0..100)
        .find_map(|noun| {
            (0..100).find_map(|verb| {
                if run_params(&mut Computer::new(input, None), noun, verb).unwrap() == OUTPUT {
                    Some(noun * 100 + verb)
                } else {
                    None
                }
            })
        })
        .unwrap()
}
