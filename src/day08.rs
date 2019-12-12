use itertools::Itertools;
use std::{env, fs::File, iter::FromIterator, path::PathBuf};

const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;
const IMG_SIZE: usize = IMG_WIDTH * IMG_HEIGHT;

#[derive(Default, Debug)]
struct Numbers {
    zeros: usize,
    ones: usize,
    twos: usize,
}

fn count<I: Iterator<Item = u8>>(i: I) -> Numbers {
    let (z, o, t) = i.fold((0, 0, 0), |(z, o, t), n| match n {
        0 => (z + 1, o, t),
        1 => (z, o + 1, t),
        2 => (z, o, t + 1),
        _ => (z, o, t),
    });
    Numbers {
        zeros: z,
        ones: o,
        twos: t,
    }
}

#[aoc(day08, part1)]
pub fn day08_part1(input: &str) -> usize {
    let numbers = input
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .chunks(IMG_SIZE)
        .into_iter()
        .fold(
            Numbers {
                zeros: usize::max_value(),
                ones: usize::max_value(),
                twos: usize::max_value(),
            },
            |numbers, layer| {
                let n = count(layer);
                if n.zeros < numbers.zeros {
                    n
                } else {
                    numbers
                }
            },
        );
    numbers.ones * numbers.twos
}

#[aoc(day08, part2)]
pub fn day08_part2(input: &str) -> String {
    let layers: Vec<Vec<u8>> = input
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .chunks(IMG_SIZE)
        .into_iter()
        .map(FromIterator::from_iter)
        .collect();
    (0..IMG_SIZE)
        .map(|i| {
            layers
                .iter()
                .fold(2, |pixel, layer| if pixel == 2 { layer[i] } else { pixel })
        })
        .chunks(IMG_WIDTH)
        .into_iter()
        .fold(String::with_capacity(IMG_SIZE), |mut res, row| {
            res.push('\n');
            row.for_each(|pixel| {
                res.push(match pixel {
                    0 => '░',
                    1 => '█',
                    _ => '?',
                })
            });
            res
        })
}
