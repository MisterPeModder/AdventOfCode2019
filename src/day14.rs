use regex::{Captures, Regex};

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Reaction<'a> {
    outputs: usize,
    inputs: Vec<(&'a str, usize)>,
}

#[inline]
fn parse_chemical<'a>(input: &'a str) -> (&'a str, usize) {
    lazy_static! {
        static ref CHEMICAL_PATTERN: Regex =
            Regex::new(r"(?P<amount>\d+)\s*(?P<name>[A-Z]+)").unwrap();
    }
    let captures: Captures<'a> = CHEMICAL_PATTERN.captures(input).expect("invalid chemical");
    (
        captures.name("name").unwrap().as_str(),
        captures["amount"].parse().unwrap(),
    )
}

fn parse_reactions<'a>(input: &'a str) -> HashMap<&'a str, Reaction<'a>> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("=>");
            let inputs = split
                .next()
                .unwrap()
                .split(',')
                .map(|input| parse_chemical(input))
                .collect();
            let output = parse_chemical(split.next().unwrap());
            (
                output.0,
                Reaction {
                    inputs,
                    outputs: output.1,
                },
            )
        })
        .collect()
}

fn do_reaction<'a>(
    output: &'a str,
    reactions: &HashMap<&'a str, Reaction<'a>>,
    stock: &mut HashMap<&'a str, usize>,
) -> usize {
    let mut ore_required = 0;
    let reaction = reactions.get(output).expect("invalid output ingredient");

    for &(name, amount) in reaction.inputs.iter() {
        if name == "ORE" {
            ore_required += amount;
        } else {
            loop {
                let in_stock = stock.entry(name).or_default();

                if *in_stock < amount {
                    ore_required += do_reaction(name, reactions, stock);
                } else {
                    *in_stock -= amount;
                    break;
                }
            }
        }
    }
    stock
        .entry(output)
        .and_modify(|c| *c += reaction.outputs)
        .or_insert(reaction.outputs);
    ore_required
}

#[aoc(day14, part1)]
pub fn day14_part1(input: &str) -> usize {
    let reactions = parse_reactions(input);
    let mut stock = HashMap::new();

    do_reaction("FUEL", &reactions, &mut stock)
}

// The Brute-force way, takes about 30 minutes to find the solution.
#[aoc(day14, part2)]
pub fn day14_part2(input: &str) -> usize {
    const ORE_THRESHOLD: usize = 1_000_000_000_000;
    let reactions = parse_reactions(input);
    let mut stock = HashMap::new();
    let mut ore = 0;
    let mut fuel = 0;

    loop {
        ore += do_reaction("FUEL", &reactions, &mut stock);
        if ore >= ORE_THRESHOLD {
            break fuel;
        }
        if fuel % 1000 == 0 {
            println!(
                "{:.3}% - fuel: {}, ore: {}/{}",
                ore as f64 / ORE_THRESHOLD as f64 * 100.0,
                fuel,
                ore,
                ORE_THRESHOLD
            );
        }
        fuel += 1;
    }
}
