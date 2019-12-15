use crate::util::computer::Computer;

use std::{collections::BTreeMap, slice::Iter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Wall = 0,
    Empty,
    Oxygen,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Tile {
        match value {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            t => panic!("found invalid tile: {}", t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North = 1,
    South,
    West,
    East,
}

impl Direction {
    #[inline]
    fn offset(self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }

    #[inline]
    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    #[inline]
    fn iter() -> Iter<'static, Direction> {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        DIRECTIONS.iter()
    }
}

fn map_to_string(map: &mut BTreeMap<(i64, i64), Tile>, pos: (i64, i64)) -> String {
    let min_x = map
        .keys()
        .map(|&(_, x)| x)
        .fold(i64::max_value(), |min, x| if x < min { x } else { min });
    let mut last_y = i64::min_value();
    let mut last_x = min_x;
    let mut res = String::new();

    for (&(y, x), tile) in map.iter() {
        if last_y != y {
            res.push('\n');
            for _ in min_x..x {
                res.push(' ');
            }
        } else {
            for _ in (last_x + 1)..x {
                res.push(' ');
            }
        }
        if x == pos.0 && y == pos.1 {
            res.push('D');
        } else if x == 0 && y == 0 {
            res.push('X');
        } else {
            res.push(match tile {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Oxygen => 'o',
            })
        }
        last_y = y;
        last_x = x;
    }
    res
}

fn explore(
    program: &mut Computer,
    map: &mut BTreeMap<(i64, i64), Tile>,
    pos: (i64, i64),
) -> Option<usize> {
    let mut steps: Option<usize> = None;

    for &direction in Direction::iter() {
        let to = direction.offset(pos);

        if map.get(&(to.1, to.0)).is_none() {
            let tile = Tile::from(program.resume_get(Some(direction as i64)).unwrap());

            map.insert((to.1, to.0), tile);
            if tile == Tile::Oxygen {
                steps = Some(0);
            }
            if tile == Tile::Empty || tile == Tile::Oxygen {
                if let Some(nsteps) = explore(program, map, to) {
                    steps = match steps {
                        Some(s) if nsteps < s => Some(nsteps),
                        None => Some(nsteps),
                        _ => steps,
                    };
                }
                // move back to original pos
                program
                    .resume_get(Some(direction.opposite() as i64))
                    .unwrap();
            }
        }
    }
    steps.map(|s| s + 1)
}

fn spread_oxygen(map: &mut BTreeMap<(i64, i64), Tile>) -> usize {
    let oxygen_tiles: Vec<(i64, i64)> = map
        .iter()
        .filter(|&(_, &t)| t == Tile::Oxygen)
        .map(|(&pos, _)| pos)
        .collect();
    let mut filled = 0;

    for pos in oxygen_tiles {
        for &direction in Direction::iter() {
            if let Some(tile) = map.get_mut(&direction.offset(pos)) {
                if *tile == Tile::Empty {
                    *tile = Tile::Oxygen;
                    filled += 1;
                }
            }
        }
    }
    filled
}

fn fill_with_oxygen(map: &mut BTreeMap<(i64, i64), Tile>) -> usize {
    let mut minutes = 0;

    while spread_oxygen(map) > 0 {
        minutes += 1;
    }
    minutes
}

#[aoc_generator(day15)]
pub fn day15_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

const MEMORY_SIZE: usize = 4096;

#[aoc(day15, part1)]
pub fn day15_part1(code: &[i64]) -> Option<usize> {
    let mut program = Computer::new(code, Some(MEMORY_SIZE));
    let mut map = BTreeMap::new();
    map.insert((0, 0), Tile::Empty);
    let route_length = explore(&mut program, &mut map, (0, 0));

    println!("{}", map_to_string(&mut map, (0, 0)));
    route_length
}

#[aoc(day15, part2)]
pub fn day15_part2(code: &[i64]) -> usize {
    let mut program = Computer::new(code, Some(MEMORY_SIZE));
    let mut map = BTreeMap::new();

    explore(&mut program, &mut map, (0, 0));
    let minutes = fill_with_oxygen(&mut map);

    println!("{}", map_to_string(&mut map, (0, 0)));
    minutes
}
