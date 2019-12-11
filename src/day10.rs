use itertools::Itertools;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::*;

pub struct Grid {
    size: u32,
    grid: Vec<bool>,
}

#[derive(Debug, Copy, Clone)]
struct OrderedFloat(ordered_float::OrderedFloat<f64>);

impl Hash for OrderedFloat {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &OrderedFloat) -> bool {
        (self.0.into_inner() - other.0.into_inner()).abs() <= 0.001
    }
}

impl Eq for OrderedFloat {}

impl Grid {
    pub fn from_input(input: &str) -> Grid {
        let grid: Vec<bool> = input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '#' => true,
                    '.' => false,
                    c => panic!("Invalid input: '{}', code {}", c, c as u32),
                })
            })
            .collect();
        let size = input.lines().nth(0).unwrap().len() as u32;
        println!("size: {}", size);
        Grid { grid, size }
    }

    #[inline]
    pub fn size(&self) -> u32 {
        self.size
    }

    #[inline]
    fn at(&self, x: u32, y: u32) -> bool {
        self.grid[(y * self.size + x) as usize]
    }

    pub fn detect_from(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let orig = (x as f64, y as f64);
        let mut points: HashMap<OrderedFloat, (u32, u32)> = HashMap::new();
        for ty in 0..self.size() {
            for tx in (0..self.size()).filter(|&tx| self.at(tx, ty) && (tx, ty) != (x, y)) {
                let t = (tx as f64, ty as f64);
                let mut angle = ((t.1 - orig.1).abs() / (t.0 - orig.0).abs()).atan();
                if ty < y {
                    angle += std::f64::consts::PI;
                }
                points
                    .entry(OrderedFloat(ordered_float::OrderedFloat(angle)))
                    .and_modify(|curr| {
                        if dist(orig, t) < dist(orig, (curr.0 as f64, curr.1 as f64)) {
                            *curr = (tx, ty);
                        }
                    })
                    .or_insert((tx, ty));
            }
        }
        points.values().copied().collect()
    }
}

#[inline]
fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx.mul_add(dx, dy * dy)).sqrt()
}

#[aoc(day10, part1)]
pub fn day10_part1(_input: &str) -> u32 {
    //let input = "#..\n.#.\n..#\n";
    let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
    let grid = Grid::from_input(input);
    (0u32..grid.size()).for_each(|y| {
        (0..grid.size()).filter(|&x| grid.at(x, y)).for_each(|x| {
            let points = grid.detect_from(x, y);
            println!("({:2}, {:2}) -> points: {:?}", x, y, points.len());
        })
    });
    0
}
