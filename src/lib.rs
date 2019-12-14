#![allow(clippy::trivially_copy_pass_by_ref, unused_imports)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate num_integer;
extern crate ordered_float;
extern crate regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

pub mod util;

aoc_lib! { year = 2019 }
