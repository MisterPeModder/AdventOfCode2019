use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day06)]
pub fn day06_gen(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            l.split(')')
                .take(2)
                .map(|s| s.to_owned())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn count_orbits(map: &HashMap<&str, Vec<&str>>, from: &str, level: usize) -> usize {
    match map.get(from) {
        Some(satellites) => {
            satellites.len() * level
                + satellites
                    .iter()
                    .map(|s| count_orbits(map, s, level + 1))
                    .sum::<usize>()
        }
        None => 0,
    }
}

#[aoc(day06, part1)]
pub fn day06_part1(input: &[(String, String)]) -> usize {
    count_orbits(
        &input
            .iter()
            .unique()
            .map(|(o, _)| {
                (
                    o as &str,
                    input
                        .iter()
                        .filter(|(o2, _)| *o == *o2)
                        .map(|(_, s)| s as &str)
                        .collect::<Vec<&str>>(),
                )
            })
            .collect::<HashMap<&str, Vec<&str>>>(),
        "COM",
        1,
    )
}

fn find_neighbors<'a>(input: &'a [(String, String)], node: &'a str) -> HashSet<&'a str> {
    input
        .iter()
        .filter(|(a, b)| *a == node || *b == node)
        .map(|(a, b)| if *a == node { b as &str } else { a as &str })
        .collect()
}

fn find_path_len<'a>(
    input: &'a [(String, String)],
    here: &'a str,
    goal: &'a str,
    visited: &mut HashSet<&'a str>,
) -> Option<usize> {
    if visited.contains(here) {
        return None;
    }
    visited.insert(here);
    if here != goal {
        find_neighbors(input, here)
            .iter()
            .map(|v| find_path_len(input, v, goal, visited))
            .find(|o| o.is_some())
            .unwrap_or(None)
            .map(|i| i + 1)
    } else {
        Some(1)
    }
}

#[aoc(day06, part2)]
pub fn day06_part2(input: &[(String, String)]) -> usize {
    find_path_len(input, "YOU", "SAN", &mut HashSet::new()).unwrap() - 3
}
