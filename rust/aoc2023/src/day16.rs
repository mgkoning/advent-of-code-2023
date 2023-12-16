use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use itertools::Itertools;

use crate::util::{assign_coordinates, Coord, Direction};

#[derive(Clone, Copy)]
struct Beam(Coord, Direction);

impl Direction {
    fn show(&self) -> char {
        match self {
            Self::North => '^',
            Self::East => '>',
            Self::South => 'v',
            Self::West => '<',
        }
    }
}

const SHOW_MAP: bool = false;

pub fn run(input: &str) -> Result<(), String> {
    let mirror_cave = read_cave(input);
    let part1 = part1(&mirror_cave);
    println!("Part 1: {part1}");
    let part2 = power_up(&mirror_cave);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(mirror_cave: &HashMap<Coord, char>) -> usize {
    let energized = energize(&mirror_cave, Beam(Coord::origin(), Direction::East));
    if SHOW_MAP {
        println!("{}", show_beams(mirror_cave, &energized))
    }
    energized.keys().count()
}

fn power_up(mirror_cave: &HashMap<Coord, char>) -> usize {
    let (max_x, max_y) = mirror_cave
        .keys()
        .fold((0, 0), |(mx, my), Coord { x, y }| (mx.max(*x), my.max(*y)));
    let (winner, power) = (0..=max_x)
        .map(|x| Beam(Coord::from(x, 0), Direction::South))
        .chain((0..=max_x).map(|x| Beam(Coord::from(x, max_y), Direction::North)))
        .chain((0..=max_y).map(|y| Beam(Coord::from(0, y), Direction::East)))
        .chain((0..=max_y).map(|y| Beam(Coord::from(max_x, y), Direction::West)))
        .map(|b| (b, energize(mirror_cave, b).keys().count()))
        .max_by_key(|(_, power)| *power)
        .unwrap();
    if SHOW_MAP {
        println!(
            "{}",
            show_beams(mirror_cave, &energize(mirror_cave, winner))
        )
    }
    power
}

fn energize(
    mirror_cave: &HashMap<Coord, char>,
    start_beam: Beam,
) -> HashMap<Coord, HashSet<Direction>> {
    let mut energized = HashMap::<Coord, HashSet<Direction>>::new();
    let mut add_beam = |Beam(pos, dir)| -> bool {
        let directions = energized.entry(pos).or_insert(HashSet::new());
        directions.insert(dir)
    };
    add_beam(start_beam);
    // Iterate beams: from every active beam, travel from its current location based on the
    // obstacle it's at. The beam becomes inactive once it reaches the cave wall or merges
    // with an existing beam.
    successors(Some(vec![start_beam]), |beams| {
        let new_beams = beams
            .iter()
            .flat_map(|b| travel(mirror_cave, b).into_iter())
            .filter(|b @ Beam(pos, _)| mirror_cave.contains_key(pos) && add_beam(*b))
            .collect_vec();
        Some(new_beams).filter(|b| !b.is_empty())
    })
    .last();
    energized
}

fn travel(mirror_cave: &HashMap<Coord, char>, beam @ Beam(pos, direction): &Beam) -> Vec<Beam> {
    mirror_cave
        .get(pos)
        .map(|c| match (c, direction) {
            // Straight through (or no obstacle)
            ('-', Direction::East)
            | ('-', Direction::West)
            | ('|', Direction::North)
            | ('|', Direction::South)
            | ('.', _) => vec![step(beam, |d| *d)],
            // Split
            ('|', Direction::East)
            | ('|', Direction::West)
            | ('-', Direction::North)
            | ('-', Direction::South) => {
                vec![
                    step(beam, Direction::turn_left),
                    step(beam, Direction::turn_right),
                ]
            }
            // Mirrors
            ('\\', Direction::North)
            | ('/', Direction::East)
            | ('\\', Direction::South)
            | ('/', Direction::West) => vec![step(beam, Direction::turn_left)],
            ('/', Direction::North)
            | ('\\', Direction::East)
            | ('/', Direction::South)
            | ('\\', Direction::West) => vec![step(beam, Direction::turn_right)],
            (_, _) => panic!("Did not expect {c}"),
        })
        .unwrap_or_else(|| vec![])
}

fn step(Beam(pos, direction): &Beam, turn: impl Fn(&Direction) -> Direction) -> Beam {
    let new_direction = turn(direction);
    Beam(pos.go(new_direction), new_direction)
}

fn read_cave(input: &str) -> HashMap<Coord, char> {
    assign_coordinates(input).collect::<HashMap<_, _>>()
}

fn show_beams(
    mirror_cave: &HashMap<Coord, char>,
    beams: &HashMap<Coord, HashSet<Direction>>,
) -> String {
    let (max_x, max_y) = mirror_cave
        .keys()
        .fold((0, 0), |(mx, my), Coord { x, y }| (mx.max(*x), my.max(*y)));
    (0..=max_y)
        .map(|line| {
            String::from_iter((0..=max_x).map(|col| Coord::from(col, line)).map(|c| {
                let char = mirror_cave
                    .get(&c)
                    .filter(|t| **t != '.')
                    .copied()
                    .or_else(|| {
                        beams.get(&c).map(|d| {
                            if 1 < d.len() {
                                d.len().to_string().chars().next().unwrap()
                            } else {
                                d.iter().next().map(Direction::show).unwrap()
                            }
                        })
                    })
                    .unwrap_or('.');
                colorize(char, beams.get(&c).map(|v| v.len()).unwrap_or(0))
            }))
        })
        .join("\n")
}
fn colorize(token: char, brightness: usize) -> String {
    // https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
    let color = match brightness {
        4 => "231",
        3 => "226",
        2 => "214",
        1 => "202",
        _ => "8",
    };
    format!("\x1b[38;5;{color}m{token}\x1b[0m")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_test() {
        assert_eq!(46, part1(&read_cave(SAMPLE_INPUT)));
    }

    #[test]
    fn part2_test() {
        assert_eq!(51, power_up(&read_cave(SAMPLE_INPUT)));
    }

    #[test]
    fn show_beams_test() {
        let cave = read_cave(SAMPLE_INPUT);
        println!(
            "{}",
            show_beams(
                &cave,
                &energize(&cave, Beam(Coord::origin(), Direction::East))
            )
        );
    }
}
