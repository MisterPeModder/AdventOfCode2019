use crate::util::computer::Computer;

use itertools::Itertools;
use std::collections::BTreeMap;

const MEMORY_SIZE: usize = 2048;

#[derive(Debug)]
struct Robot {
    brain: Computer,
    facing: Facing,
    pos: (i64, i64),
}

impl Robot {
    #[inline]
    fn new(code: &[i64]) -> Robot {
        Robot {
            brain: Computer::new(code, Some(MEMORY_SIZE)),
            facing: Facing::Up,
            pos: (0, 0),
        }
    }

    fn next(&mut self, panel: &mut Panel) -> bool {
        //println!("{:?}", self.pos);
        match self.brain.resume(Some(panel.color as i64)).unwrap() {
            Some(new_color) => {
                panel.color = match new_color {
                    0 => Color::Black,
                    1 => Color::White,
                    c => panic!("Invalid color: {}", c),
                };
                panel.painted = true;
                match self.brain.resume_get(None).unwrap() {
                    0 => self.facing = self.facing.left(),
                    1 => self.facing = self.facing.right(),
                    t => panic!("Invalid turn: {}", t),
                }
                self.pos = self.facing.forward(self.pos);
                true
            }
            None => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    #[inline]
    fn left(self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }

    #[inline]
    fn right(self) -> Facing {
        match self {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
        }
    }

    fn forward(self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Facing::Up => (x, y + 1),
            Facing::Left => (x + 1, y),
            Facing::Down => (x, y - 1),
            Facing::Right => (x - 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
enum Color {
    Black = 0,
    White,
}

#[derive(Debug, Clone)]
struct Panel {
    color: Color,
    painted: bool,
}

impl Panel {
    #[inline]
    fn black() -> Panel {
        Panel {
            color: Color::Black,
            painted: false,
        }
    }
}

#[derive(Debug)]
struct Hull {
    panels: BTreeMap<i64, BTreeMap<i64, Panel>>,
}

impl Hull {
    fn new() -> Hull {
        Hull {
            panels: BTreeMap::new(),
        }
    }
}

impl Hull {
    fn panel_mut(&mut self, (x, y): (i64, i64)) -> &mut Panel {
        self.panels
            .entry(y)
            .or_insert_with(BTreeMap::new)
            .entry(x)
            .or_insert_with(Panel::black)
    }
}

#[aoc_generator(day11)]
pub fn day10_gen(input: &str) -> Vec<i64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
pub fn day11_part1(input: &[i64]) -> usize {
    let mut hull = Hull::new();
    let mut robot = Robot::new(input);
    while robot.next(hull.panel_mut(robot.pos)) {}
    hull.panels
        .values()
        .map(|panels| panels.values().filter(|panel| panel.painted).count())
        .sum()
}

#[aoc(day11, part2)]
pub fn day11_part2(input: &[i64]) -> String {
    let mut hull = Hull::new();
    let mut robot = Robot::new(input);
    hull.panel_mut((0, 0)).color = Color::White;
    while robot.next(hull.panel_mut(robot.pos)) {}
    let x_min = hull
        .panels
        .values()
        .map(|panels| panels.keys().next().unwrap())
        .sorted()
        .next()
        .unwrap();
    let x_max = hull
        .panels
        .values()
        .map(|panels| panels.keys().last().unwrap())
        .sorted()
        .last()
        .unwrap();

    let mut row = vec![Color::Black; (x_max - x_min) as usize + 1];
    let mut res = String::with_capacity(hull.panels.len() * row.len());
    res.push('\n');
    for panel_row in hull.panels.values().rev() {
        panel_row
            .iter()
            .for_each(|(x, panel)| row[*x as usize] = panel.color);
        row.iter_mut().for_each(|c| {
            match c {
                Color::Black => res.push('░'),
                Color::White => res.push('█'),
            };
            *c = Color::Black;
        });
        res.push('\n');
    }
    res
}
