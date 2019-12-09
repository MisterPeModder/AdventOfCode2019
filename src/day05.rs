use crate::util::computer::Computer;

#[aoc_generator(day05)]
pub fn day05_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day05, part1)]
pub fn day05_part1(input: &[i64]) -> i64 {
    Computer::new(input, None)
        .resume_iter(std::iter::once(1))
        .map(|o| o.unwrap())
        .last()
        .unwrap()
}

#[aoc(day05, part2)]
pub fn day05_part2(input: &[i64]) -> i64 {
    Computer::new(input, None).resume(Some(5)).unwrap().unwrap()
}
