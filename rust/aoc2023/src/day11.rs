use std::collections::HashSet;

use itertools::Itertools;

use crate::util::{assign_coordinates, Coord};

pub fn run(input: &str) -> Result<(), String> {
    let read = read_input(input);
    let part1 = distance_after_expand(&read, 2);
    println!("Part 1: {part1}");
    let part2 = distance_after_expand(&read, 1_000_000);
    println!("Part 2: {part2}");
    Ok(())
}

fn distance_after_expand(galaxies: &Vec<Coord>, factor: i64) -> i64 {
    let expanded = expand(galaxies, factor);
    expanded
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.manhattan_distance(b))
        .sum()
}

fn expand(galaxies: &Vec<Coord>, factor: i64) -> Vec<Coord> {
    let list_missing = |zs: HashSet<i64>| {
        (0..*zs.iter().max().unwrap_or(&0))
            .filter(|z| !zs.contains(z))
            .collect_vec()
    };
    let count_smaller = |zs: &Vec<_>, limit| zs.iter().filter(|z| **z < limit).count() as i64;
    let (xs, ys): (HashSet<_>, HashSet<_>) = galaxies.iter().map(|c| (c.x, c.y)).unzip();
    let expand_x = list_missing(xs);
    let expand_y = list_missing(ys);
    galaxies
        .iter()
        .map(|coord| Coord {
            x: coord.x + (factor - 1) * count_smaller(&expand_x, coord.x),
            y: coord.y + (factor - 1) * count_smaller(&expand_y, coord.y),
        })
        .collect_vec()
}

fn read_input(input: &str) -> Vec<Coord> {
    assign_coordinates(input)
        .filter(|(_, c)| *c != '.')
        .map(|p| p.0)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_test() {
        assert_eq!(374, distance_after_expand(&read_input(SAMPLE_INPUT), 2));
    }

    #[test]
    fn part2_test() {
        assert_eq!(8410, distance_after_expand(&read_input(SAMPLE_INPUT), 100));
    }
}
