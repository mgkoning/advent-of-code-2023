#![feature(slice_as_chunks, iter_map_windows)]
use std::{
    env::{self, Args},
    fs,
    num::ParseIntError,
};

use chrono::Datelike;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;

fn main() -> Result<(), String> {
    let puzzle = get_puzzle(env::args()).map_err(|e| e.to_string())?;
    let day_runner = get_day_runner(puzzle).ok_or(format!("Day {puzzle} not supported"))?;
    let input = get_input(puzzle)?;
    time(|| day_runner(&input))
}

fn time<T>(run: impl Fn() -> T) -> T {
    let start = std::time::Instant::now();
    let result = run();
    println!("Elapsed: {:?}", start.elapsed());
    result
}

type DayRunner = fn(&str) -> Result<(), String>;
const RUNNERS: [DayRunner; 7] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
];
fn get_day_runner(puzzle: u32) -> Option<DayRunner> {
    RUNNERS.get(puzzle as usize - 1).copied()
}

fn get_puzzle(args: Args) -> Result<u32, ParseIntError> {
    args.skip(1)
        .next()
        .map(|s| s.parse())
        .unwrap_or_else(|| Ok(chrono::Local::now().date_naive().day()))
}

fn get_input(puzzle: u32) -> Result<String, String> {
    let filename = format!("day{:02}.txt", puzzle);
    fs::read_to_string(format!("../../input/{filename}")).map_err(|e| e.to_string())
}
