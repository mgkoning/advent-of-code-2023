use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{assign_coordinates, Coord};

#[derive(Debug, Clone)]
struct Platform {
    size: (i64, i64),
    rocks: HashMap<Coord, char>,
}

pub fn run(input: &str) -> Result<(), String> {
    let platform = read_input(input);
    let part1 = part1(&platform);
    println!("Part 1: {part1}");
    let part2 = part2(&platform);
    println!("Part 2: {part2}");

    Ok(())
}

fn part1(platform: &Platform) -> i64 {
    let tilted = tilt(&platform, (0, -1));
    load(&tilted)
}

fn load(platform: &Platform) -> i64 {
    platform
        .rocks
        .iter()
        .filter_map(|(Coord { y, .. }, rock)| {
            Some(rock)
                .filter(|r| **r == 'O')
                .map(|_| platform.size.1 - y)
        })
        .sum()
}

fn part2(platform: &Platform) -> i64 {
    const CYCLE_COUNT: i64 = 1_000_000_000;
    let mut seen = HashMap::<String, (i64, usize)>::new();
    let cycles = (0..CYCLE_COUNT)
        .scan(platform.clone(), |p, _| {
            let tilted = spin_cycle(&p);
            let result = (load(&tilted), to_map(&tilted));
            *p = tilted;
            Some(result)
        })
        .enumerate()
        .take_while_inclusive(|(i, (load, map))| {
            if seen.contains_key(map) {
                false
            } else {
                seen.insert(map.to_owned(), (*load, *i));
                true
            }
        })
        .map(|(_, load)| load)
        .collect_vec();
    let ((_, from), to) = (
        *seen.get(&cycles.last().unwrap().1).unwrap(),
        cycles.len() - 1,
    );
    let cycle_length = to - from;
    let load_index = (CYCLE_COUNT - from as i64) % cycle_length as i64;
    cycles.get(from - 1 + load_index as usize).unwrap().0
}

fn spin_cycle(platform: &Platform) -> Platform {
    [(0, -1), (-1, 0), (0, 1), (1, 0)]
        .iter()
        .fold(platform.clone(), |p: Platform, dir| tilt(&p, *dir))
}

fn tilt(Platform { size, rocks }: &Platform, direction: (i64, i64)) -> Platform {
    let mut new_rocks = rocks.clone();
    move_order(direction, *size)
        .iter()
        .for_each(|coord| move_rock(&mut new_rocks, &coord, direction, *size));

    Platform {
        size: *size,
        rocks: new_rocks,
    }
}

fn move_order(direction: (i64, i64), (size_x, size_y): (i64, i64)) -> Vec<Coord> {
    match direction {
        (0, dy) if dy < 0 => (0..size_y)
            .flat_map(move |y| (0..size_x).map(move |x| Coord::from(x, y)))
            .collect_vec(),
        (0, _dy) => (0..size_y)
            .rev()
            .flat_map(move |y| (0..size_x).map(move |x| Coord::from(x, y)))
            .collect_vec(),
        (dx, 0) if dx < 0 => (0..size_x)
            .flat_map(move |x| (0..size_y).map(move |y| Coord::from(x, y)))
            .collect_vec(),
        (_dx, 0) => (0..size_x)
            .rev()
            .flat_map(move |x| (0..size_y).map(move |y| Coord::from(x, y)))
            .collect_vec(),
        _ => panic!("Not a direction: {:?}", direction),
    }
}

fn move_rock(
    rocks: &mut HashMap<Coord, char>,
    coord: &Coord,
    (dir_x, dir_y): (i64, i64),
    (size_x, size_y): (i64, i64),
) {
    let within_bounds =
        |coord: &Coord| 0 <= coord.x && coord.x < size_x && 0 <= coord.y && coord.y < size_y;
    match rocks.get(coord) {
        Some('O') => {
            rocks.remove(coord);
            let new_position = (0..)
                .map(|n| coord.move_by((dir_x * n, dir_y * n)))
                .take_while(|coord| within_bounds(&coord) && !rocks.contains_key(coord))
                .last()
                .unwrap_or(*coord);
            rocks.insert(new_position, 'O');
        }
        _ => {}
    }
}

fn read_input(input: &str) -> Platform {
    let (rocks, (max_x, max_y)) = assign_coordinates(input).fold(
        (HashMap::new(), (-1, -1)),
        |(mut rocks, (max_x, max_y)), (coord, rock)| {
            if rock != '.' {
                rocks.insert(coord, rock);
            }
            (rocks, (max_x.max(coord.x), max_y.max(coord.y)))
        },
    );
    Platform {
        size: (max_x + 1, max_y + 1),
        rocks,
    }
}

fn to_map(platform: &Platform) -> String {
    (0..platform.size.1)
        .map(|line| {
            (0..platform.size.0)
                .map(|col| platform.rocks.get(&Coord::from(col, line)).unwrap_or(&'.'))
                .join("")
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_test() {
        assert_eq!(136, part1(&read_input(SAMPLE_INPUT)))
    }

    #[test]
    fn spin_cycle_test() {
        (1..4).fold(read_input(SAMPLE_INPUT), |p, _| {
            let r = spin_cycle(&p);
            println!("{}", to_map(&r));
            println!();
            r
        });
    }

    #[test]
    fn part2_test() {
        assert_eq!(64, part2(&read_input(SAMPLE_INPUT)));
    }
}
