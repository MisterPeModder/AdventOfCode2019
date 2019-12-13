use crate::util::computer::{Computer, ResumeIter};

use itertools::Itertools;
use std::io::{self, prelude::*};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Empty = 0,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Tile {
        match value {
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => Tile::Empty,
        }
    }
}

const MEMORY_SIZE: usize = 4096;
const GRID_WIDTH: usize = 46;
const GRID_HEIGHT: usize = 26;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;

#[aoc_generator(day13)]
pub fn day13_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day13, part1)]
pub fn day13_part1(code: &[i64]) -> usize {
    let grid: Vec<Tile> = Computer::new(code, Some(MEMORY_SIZE))
        .resume_iter(None.into_iter())
        .map(|v| v.unwrap())
        .chunks(3)
        .into_iter()
        .fold(vec![Tile::Empty; GRID_SIZE], |mut grid, mut data| {
            let x = data.next().unwrap() as usize;
            let y = data.next().unwrap() as usize;
            let tile = Tile::from(data.next().unwrap() as u8);
            grid[x + y * GRID_WIDTH] = tile;
            grid
        });
    grid.iter().fold(0, |count, &tile| {
        if tile == Tile::Block {
            count + 1
        } else {
            count
        }
    })
}

fn print_grid(grid: &[Tile]) {
    print!(
        "{}",
        grid.iter().chunks(GRID_WIDTH).into_iter().fold(
            String::with_capacity(GRID_SIZE + GRID_HEIGHT),
            |mut res, row| {
                row.for_each(|&tile| {
                    res.push(match tile {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Block => 'x',
                        Tile::Paddle => '_',
                        Tile::Ball => 'o',
                    });
                });
                res.push('\n');
                res
            },
        )
    );
}

#[aoc(day13, part2)]
pub fn day13_part2(code: &[i64]) -> usize {
    let mut game = Computer::new(code, Some(MEMORY_SIZE));
    let mut input: std::option::IntoIter<i64> = None.into_iter();
    let mut score = 0;
    let mut grid = vec![Tile::Empty; GRID_SIZE];
    let mut ball_pos = (-1, -1);

    game.write_raw(0, 2).unwrap();
    loop {
        if input.len() == 0 {
            let buf = &mut [0, 0, 0];

            print!("score: {}\n(l/n/r): ", score);
            io::stdout().flush().unwrap();
            io::stdin().read_exact(buf).unwrap();
            input = Some(match buf[0] {
                b'l' => -1,
                b'n' => 0,
                b'r' => 1,
                i => panic!("unexpected input: {}", i),
            })
            .into_iter();
        }
        match game.resume(&mut input).unwrap() {
            Some(o1) => {
                let o2 = game.resume_get(None.into_iter()).unwrap();
                let o3 = game.resume_get(None.into_iter()).unwrap();

                if o1 == -1 && o2 == 0 {
                    println!("score!");
                    score = o3 as usize;
                } else {
                    let tile = Tile::from(o3 as u8);
                    print!("store: ({}, {}) = {:?}, ", o1, o2, tile);
                    if ball_pos != (o1, o2) || tile != Tile::Empty {
                        if tile == Tile::Ball || tile == Tile::Paddle {
                            print_grid(&grid);
                            std::thread::sleep(std::time::Duration::from_millis(100));
                        }
                        grid[o1 as usize + o2 as usize * GRID_WIDTH] = Tile::from(o3 as u8);
                    }
                    match tile {
                        Tile::Ball => ball_pos = (o1, o2),
                        _ => ball_pos = (-1, -1),
                    }
                }
            }
            None => break score,
        }
    }
}
