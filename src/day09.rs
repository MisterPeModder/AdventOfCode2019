use crate::util::computer::Computer;

const MEMORY_SIZE: usize = 2048;

#[aoc_generator(day09)]
pub fn day09_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day09, part1)]
pub fn day09_part1(input: &[i64]) -> i64 {
    Computer::new(input, Some(MEMORY_SIZE))
        .resume_get(Some(1))
        .unwrap()
}

#[aoc(day09, part2)]
pub fn day09_part2(input: &[i64]) -> i64 {
    Computer::new(input, Some(MEMORY_SIZE))
        .resume_get(Some(2))
        .unwrap()
}
