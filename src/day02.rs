use crate::util::computer::{self, Computer};

#[aoc_generator(day02)]
pub fn day02_gen(input: &str) -> Vec<i32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn run_params(computer: &mut Computer, noun: i32, verb: i32) -> computer::Result<i32> {
    computer.write_raw(1, noun)?;
    computer.write_raw(2, verb)?;
    computer.resume(None)?;
    computer.read_raw(0)
}

#[aoc(day02, part1)]
pub fn day02_part1(input: &[i32]) -> i32 {
    const NOUN: i32 = 12;
    const VERB: i32 = 2;

    run_params(&mut Computer::new(input), NOUN, VERB).unwrap()
}

#[aoc(day02, part2)]
pub fn day02_part2(input: &[i32]) -> i32 {
    const OUTPUT: i32 = 19_690_720;

    (0..100)
        .find_map(|noun| {
            (0..100).find_map(|verb| {
                if run_params(&mut Computer::new(input), noun, verb).unwrap() == OUTPUT {
                    Some(noun * 100 + verb)
                } else {
                    None
                }
            })
        })
        .unwrap()
}
