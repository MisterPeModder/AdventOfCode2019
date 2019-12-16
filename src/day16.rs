use std::{iter, mem};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct RepeatValues<I: Iterator> {
    max: usize,
    repeated: usize,
    iter: I,
    current: Option<I::Item>,
}

impl<I: Iterator> RepeatValues<I> {
    #[inline]
    fn new(mut iter: I, max: usize) -> RepeatValues<I> {
        let current = iter.next();
        RepeatValues {
            max,
            repeated: 1,
            iter,
            current,
        }
    }
}

impl<I> Iterator for RepeatValues<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.max == 0 {
            None
        } else if self.repeated != self.max {
            self.repeated += 1;
            self.current.clone()
        } else {
            self.repeated = 1;
            mem::replace(&mut self.current, self.iter.next())
        }
    }
}

trait IteratorExt: Iterator {
    #[inline]
    fn repeat_values(self, max: usize) -> RepeatValues<Self>
    where
        Self: Sized,
    {
        RepeatValues::new(self, max)
    }
}

impl<I, T> IteratorExt for I where I: Iterator<Item = T> {}

fn do_phase(input: &[i64], buf: &mut [i64]) {
    const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

    (1..=input.len())
        .map(|i| {
            (BASE_PATTERN
                .iter()
                .repeat_values(i)
                .cycle()
                .skip(1)
                .zip(input.iter())
                .map(|(&m, &n)| m * n as i32)
                .sum::<i32>()
                % 10)
                .abs() as i64
        })
        .enumerate()
        .for_each(|(i, n)| buf[i] = n);
}

fn repeat_phases(iterations: usize, input_buf: &mut [i64], buf: &mut [i64]) {
    let mut input = input_buf;
    let mut output = buf;

    for _ in 0..iterations {
        do_phase(input, output);
        mem::swap(&mut input, &mut output);
    }
}

#[aoc_generator(day16)]
pub fn day16_gen(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

const ITERATIONS: usize = 100;

#[aoc(day16, part1)]
pub fn day16_part1(input: &[i64]) -> String {
    let mut input = Vec::from(input);
    let mut buf = vec![10; input.len()];

    repeat_phases(ITERATIONS, &mut input, &mut buf);
    input
        .iter()
        .map(|&n| (n as u8 + b'0') as char)
        .take(8)
        .collect()
}

// The Naive Way
/*
#[aoc(day16, part2)]
pub fn day16_part2(input: &[u8]) -> String {
    let offset = input.iter().take(7).fold(0, |o, &n| o * 10 + n as usize);
    println!("offset {}", offset);
    let mut nums: Vec<u8> = input
        .iter()
        .copied()
        .cycle()
        .take(input.len() * 10_000)
        .collect();
    let mut buf = vec![10; nums.len()];
    println!("allocated buffers!\nlen: {}", nums.len());
    repeat_phases(ITERATIONS, &mut nums, &mut buf);
    println!("repeated {} times!", ITERATIONS);
    nums
        .iter()
        .skip(offset)
        .map(|n| (n + b'0') as char)
        .take(8)
        .collect()
}
*/

#[aoc(day16, part2)]
pub fn day16_part2(input: &[i64]) -> String {
    let offset = input.iter().take(7).fold(0, |o, &n| o * 10 + n as usize);

    // calculate new input
    let mut nums = std::iter::repeat(input)
        .take(10000)
        .flatten()
        .skip(offset)
        .copied()
        .collect::<Vec<_>>();

    for _ in 0..ITERATIONS {
        // reverse prefix sum
        for i in (0..nums.len() - 1).rev() {
            nums[i] += nums[i + 1];
        }

        nums.iter_mut().for_each(|x| *x %= 10);
    }

    nums.iter()
        .map(|&n| (n as u8 + b'0') as char)
        .take(8)
        .collect()
}
