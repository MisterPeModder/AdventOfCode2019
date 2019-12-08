use image::{png::PNGEncoder, ColorType, ImageBuffer};
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

// slightly overkill, but ¯\_(ツ)_/¯
#[aoc(day08, part2)]
pub fn day08_part2(input: &str) -> String {
    let layers: Vec<Vec<u8>> = input
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .chunks(IMG_SIZE)
        .into_iter()
        .map(FromIterator::from_iter)
        .collect();
    let pixels = (0..IMG_SIZE)
        .map(|i| {
            layers
                .iter()
                .fold(2, |pixel, layer| if pixel == 2 { layer[i] } else { pixel })
        })
        .map(|d| match d {
            0 => 0u8,
            1 => 255,
            _ => 100,
        })
        .collect::<Vec<u8>>();
    let mut path = PathBuf::from(env::var_os("OUTPUT_DIR").expect("missing OUTPUT_DIR variable"));
    path.push("output_day08.png");
    PNGEncoder::new(
        File::create(&path)
            .map_err(|e| {
                format!(
                    "error while creating image at {}: {}",
                    path.to_string_lossy(),
                    e
                )
            })
            .unwrap(),
    )
    .encode(
        &pixels,
        IMG_WIDTH as u32,
        IMG_HEIGHT as u32,
        ColorType::Gray(8),
    )
    .unwrap();
    format!("output path: {}", path.to_string_lossy())
}
