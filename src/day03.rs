use itertools::Itertools;

use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub struct WirePath(Vec<(Direction, i32)>);

#[aoc_generator(day03)]
pub fn day03_gen(input: &str) -> (WirePath, WirePath) {
    input
        .lines()
        .take(2)
        .map(|l| {
            WirePath(
                l.split(',')
                    .map(|d| {
                        let (direction, value) = d.split_at(1);
                        (
                            match direction {
                                "U" => Direction::Up,
                                "D" => Direction::Down,
                                "R" => Direction::Right,
                                "L" => Direction::Left,
                                _ => panic!("invalid direction"),
                            },
                            value.parse().unwrap(),
                        )
                    })
                    .collect(),
            )
        })
        .collect_tuple()
        .unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Line {
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
    steps: u32,
}

impl Line {
    fn new(steps: u32, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Line {
        Line {
            from_x,
            from_y,
            to_x,
            to_y,
            steps,
        }
    }

    fn min(&self) -> Line {
        Line {
            from_x: cmp::min(self.from_x, self.to_x),
            from_y: cmp::min(self.from_y, self.to_y),
            to_x: cmp::max(self.from_x, self.to_x),
            to_y: cmp::max(self.from_y, self.to_y),
            steps: self.steps,
        }
    }

    fn intersects_with(&self, other: &Line) -> Option<(i32, i32, u32)> {
        let l1 = self.min();
        let l2 = other.min();
        if (l1.from_x <= l2.to_x && l1.to_x >= l2.to_x)
            && (l2.from_y <= l1.to_y && l2.to_y >= l1.to_y)
        {
            let i = (l2.to_x, l1.to_y);
            Some((
                i.0,
                i.1,
                (i.0 - self.from_x).abs() as u32
                    + l1.steps
                    + (i.1 - other.from_y).abs() as u32
                    + l2.steps,
            ))
        } else if (l2.from_x <= l1.to_x && l2.to_x >= l1.to_x)
            && (l1.from_y <= l2.to_y && l1.to_y >= l2.to_y)
        {
            let i = (l1.to_x, l2.to_y);
            Some((
                i.0,
                i.1,
                (i.0 - self.from_y).abs() as u32
                    + l1.steps
                    + (i.1 - other.from_x).abs() as u32
                    + l2.steps,
            ))
        } else {
            None
        }
    }
}

fn to_lines(path: &WirePath) -> Vec<Line> {
    let mut lines = Vec::with_capacity(path.0.len());
    path.0
        .iter()
        .fold((0i32, 0i32, 0u32), |(x, y, steps), (direction, value)| {
            let to = match direction {
                Direction::Up => (x, y + *value, steps + (value.abs() as u32)),
                Direction::Down => (x, y - *value, steps + (value.abs() as u32)),
                Direction::Right => (x + *value, y, steps + (value.abs() as u32)),
                Direction::Left => (x - *value, y, steps + (value.abs() as u32)),
            };
            lines.push(Line::new(steps, x, y, to.0, to.1));
            to
        });
    lines
}

#[aoc(day03, part1)]
pub fn day03_part1(input: &(WirePath, WirePath)) -> u32 {
    let lines1 = to_lines(&input.0);
    let lines2 = to_lines(&input.1);
    lines1
        .iter()
        .flat_map(|l1| lines2.iter().filter_map(move |l2| l1.intersects_with(l2)))
        .map(|(dx, dy, _)| (dx.abs() + dy.abs()) as u32)
        .filter(|&d| d != 0)
        .sorted()
        .nth(0)
        .expect("intersection not found")
}

#[aoc(day03, part2)]
pub fn day03_part2(input: &(WirePath, WirePath)) -> u32 {
    let lines1 = to_lines(&input.0);
    let lines2 = to_lines(&input.1);
    lines1
        .iter()
        .flat_map(|l1| lines2.iter().filter_map(move |l2| l1.intersects_with(l2)))
        .map(|(_, _, steps)| steps)
        .filter(|&d| d != 0)
        .sorted()
        .nth(0)
        .expect("intersection not found")
}
