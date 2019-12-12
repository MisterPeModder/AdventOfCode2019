use itertools::Itertools;
use num_integer::Integer;
use regex::Regex;

use std::{cmp::Ordering, collections::HashMap};

#[aoc_generator(day12)]
pub fn day12_gen(input: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    lazy_static! {
        static ref PATTERN: Regex =
            Regex::new(r"<x=(?P<x>-?\d+),\s*y=(?P<y>-?\d+),\s*z=(?P<z>-?\d+)>").unwrap();
    }
    let mut vx = Vec::new();
    let mut vy = Vec::new();
    let mut vz = Vec::new();

    PATTERN.captures_iter(input).for_each(|caps| {
        vx.push(caps["x"].parse().unwrap());
        vy.push(caps["y"].parse().unwrap());
        vz.push(caps["z"].parse().unwrap());
    });
    (vx, vy, vz)
}

fn step_axis(positions: &mut Vec<i32>, velocities: &mut Vec<i32>) {
    positions
        .iter()
        .zip(velocities.iter_mut())
        .for_each(|(p1, vel)| {
            positions.iter().for_each(|p2| {
                *vel += match p1.cmp(&p2) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                }
            })
        });
    positions
        .iter_mut()
        .zip(velocities.iter())
        .for_each(|(pos, vel)| *pos += vel);
}

#[aoc(day12, part1)]
pub fn day12_part1(input: &(Vec<i32>, Vec<i32>, Vec<i32>)) -> i32 {
    const STEPS: u64 = 1000;

    let mut px = Vec::from(&input.0 as &[i32]);
    let mut py = Vec::from(&input.1 as &[i32]);
    let mut pz = Vec::from(&input.2 as &[i32]);
    let mut vx = vec![0; px.len()];
    let mut vy = vec![0; py.len()];
    let mut vz = vec![0; pz.len()];

    for _ in 0..STEPS {
        step_axis(&mut px, &mut vx);
        step_axis(&mut py, &mut vy);
        step_axis(&mut pz, &mut vz);
    }
    (0..px.len())
        .map(|i| {
            (px[i].abs() + py[i].abs() + pz[i].abs()) * (vx[i].abs() + vy[i].abs() + vz[i].abs())
        })
        .sum()
}

fn find_axis_cycle(initial_positions: &[i32]) -> u64 {
    let mut positions = Vec::from(initial_positions);
    let mut velocities = vec![0; positions.len()];
    let mut step = 1;

    loop {
        step_axis(&mut positions, &mut velocities);
        if velocities.iter().all(|&v| v == 0) && positions == initial_positions {
            break step;
        }
        step += 1;
    }
}

#[aoc(day12, part2)]
pub fn day12_part2(input: &(Vec<i32>, Vec<i32>, Vec<i32>)) -> u64 {
    let sx = find_axis_cycle(&input.0);
    let sy = find_axis_cycle(&input.1);
    let sz = find_axis_cycle(&input.2);
    sy.lcm(&sx.lcm(&sz))
}
