use std::{collections::{HashSet, VecDeque}, iter::successors};

use itertools::Itertools;

use crate::util::{read, Coord, Direction};

struct Instruction<'a> {
    command: char,
    length: u32,
    color: &'a str
}

pub fn run(input: &str) -> Result<(), String> {
    let instructions = read_instructions(input)?;
    let part1 = part1(&instructions);
    println!("Part 1: {part1}");
    let part2 = part2(&instructions);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(instructions: &Vec<Instruction>) -> usize {
    let mut trench = HashSet::from([Coord::origin()]);
    instructions.iter().fold(Coord::origin(), |at, Instruction { command, length, .. }| {
        let dug = dig(at, *command, *length);
        trench.extend(dug.iter());
        *dug.last().unwrap_or(&at)
    });
    flood_fill(&mut trench, Coord::from(1, 1));
    
    trench.len()
}

fn part2(instructions: &Vec<Instruction>) -> i64 {
    let mut trench = vec![Coord::origin()];
    instructions.iter().fold(Coord::origin(), |at, Instruction { color, .. }| {
        let length = color.chars().skip(2).take(5).map(|c| c.to_digit(16).unwrap()).fold(0, |acc, next| acc * 16 + next);
        let command = color.chars().nth(7).unwrap();
        let next = follow(at, command, length);
        trench.push(next);
        next
    });
    let dims = trench.iter()
        .fold((0, 0, 0, 0),
            |(min_x, min_y, max_x, max_y), Coord { x, y }| (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y)));
    println!("{:?}",dims);
    // this doesn't work because all (from, to) are inclusive
    let mut area = 0;
    for i in 0..trench.len() - 2 {
        let j = (i + 1) % (trench.len() - 1);
        area += trench[i].x * trench[j].y;
        area -= trench[i].y * trench[j].x;
    }
    (area / 2).abs()
}

fn flood_fill(trench: &mut HashSet<Coord>, start: Coord) {
    let mut to_do = VecDeque::from([start]);
    while let Some(next) = to_do.pop_front() {
        to_do.extend(
            [Direction::South, Direction::East, Direction::North, Direction::West].iter()
                .map(|dir| next.go(*dir))
                .filter(|neighbor| trench.insert(*neighbor)));
    }
}

fn follow(from: Coord, command: char, length: u32) -> Coord {
    let direction = match command {
        'U' | '3' => Direction::North,
        'D' | '1' => Direction::South,
        'L' | '2' => Direction::West,
        'R' | '0' => Direction::East,
        _ => panic!("Not a direction: {command}")
    };
    from.go_by(direction, length as i64)
}

fn dig(from: Coord, command: char, length: u32) -> Vec<Coord> {
    let direction = match command {
        'U' | '3' => Direction::North,
        'D' | '1' => Direction::South,
        'L' | '2' => Direction::West,
        'R' | '0' => Direction::East,
        _ => panic!("Not a direction: {command}")
    };
    successors(Some(from), move |pos| Some(pos.go(direction))).skip(1).take(length as usize).collect_vec()
}

fn read_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|l| match &l.split_whitespace().collect_vec()[..] {
        [command, length_str, color] =>
            read::<u32>(length_str).map(|length| Instruction { command: command.chars().nth(0).unwrap(), length, color }),
        _ => Err(format!("Could not read line {l}"))
    })
    .try_collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_test() {
        assert_eq!(62, part1(&read_instructions(SAMPLE_INPUT).unwrap()));
    }

    #[test]
    fn part2_test() {
        //assert_eq!(952408144115, part2(&read_instructions(SAMPLE_INPUT).unwrap()));
    }
}