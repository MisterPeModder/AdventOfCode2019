#[aoc_generator(day1)]
pub fn day01_gen(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn day01_part1(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|&n| (n as f32 / 3.0).floor() as i32 - 2)
        .sum()
}

fn calculate_fuel(n: i32) -> i32 {
    let fuel = (n as f32 / 3.0).floor() as i32 - 2;
    if fuel > 0 {
        fuel + calculate_fuel(fuel)
    } else {
        0
    }
}

#[aoc(day1, part2)]
pub fn day01_part2(input: &[i32]) -> i32 {
    input.iter().map(|&n| calculate_fuel(n)).sum()
}
