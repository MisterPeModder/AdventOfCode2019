use itertools::Itertools;
use std::iter;

#[aoc_generator(day04)]
pub fn day04_gen(input: &str) -> (u32, u32) {
    input
        .split('-')
        .take(2)
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Digits {
    n: u32,
    divisor: u32,
}

impl Digits {
    fn new(n: u32) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits { n, divisor }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}

#[inline]
fn digits(n: u32) -> Digits {
    Digits::new(n)
}

#[aoc(day04, part1)]
pub fn day04_part1(input: &(u32, u32)) -> usize {
    (input.0..input.1)
        .filter(|&num| {
            let mut digits = digits(num);
            // six digits
            digits.clone().count() <= 6
                // double adjacent digits
                && digits.clone().tuple_windows().any(|(n, m)| n == m)
                // increasing digits
                && digits
                    .try_fold(0, |last, d| if d >= last { Some(d) } else { None })
                    .is_some()
        })
        .count()
}

#[aoc(day04, part2)]
pub fn day04_part2(input: &(u32, u32)) -> usize {
    (input.0..input.1)
        .filter(|&num| {
            let mut digits = digits(num);
            // six digits
            digits.clone().count() <= 6
                // exactly double adjacent digits
                && iter::once(24)
                    .chain(digits.clone())
                    .chain(iter::once(42))
                    .tuple_windows()
                    .filter(|(_, n, m, _)| n == m)
                    .filter(|(b, n, _, a)| n != b && n != a)
                    .count()
                    > 0
                    &&
                // increasing digits
                digits
                    .try_fold(0, |last, d| if d >= last { Some(d) } else { None })
                    .is_some()
        })
        .count()
}
