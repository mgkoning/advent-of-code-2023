use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{assign_coordinates, Coord};

type Pattern = HashMap<Coord, char>;
#[derive(PartialEq, Eq, Debug)]
enum Reflection {
    Row(i64),
    Col(i64),
}

pub fn run(input: &str) -> Result<(), String> {
    let patterns = read_patterns(input);
    let part1 = part1(&patterns)?;
    println!("Part 1: {part1}");
    let part2 = part2(&patterns)?;
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(patterns: &Vec<Pattern>) -> Result<i64, String> {
    patterns
        .iter()
        .map(|p| {
            find_reflection(p, None, &None)
                .ok_or_else(|| format!("Could not determine reflection for {:?}", p))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|reflection| reflection.iter().map(reflection_summary).sum())
}

fn part2(patterns: &Vec<Pattern>) -> Result<i64, String> {
    patterns
        .iter()
        .map(|p| {
            let unsmudged_reflection = find_reflection(p, None, &None);
            p.keys()
                .find_map(|c| find_reflection(p, Some(*c), &unsmudged_reflection))
                .ok_or_else(|| format!("Could not determine reflection for {:?}", p))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|reflection| reflection.iter().map(reflection_summary).sum())
}

fn reflection_summary(reflection: &Reflection) -> i64 {
    match reflection {
        Reflection::Row(n) => n * 100,
        Reflection::Col(n) => *n,
    }
}

fn invert(terrain: &char) -> char {
    match *terrain {
        '#' => '.',
        _ => '#',
    }
}

fn find_reflection(
    pattern: &Pattern,
    smudge: Option<Coord>,
    skip: &Option<Reflection>,
) -> Option<Reflection> {
    let get_terrain = |coord| {
        let terrain = pattern.get(&coord).unwrap();
        let result = smudge
            .filter(|c| *c == coord)
            .map(|_| invert(terrain))
            .unwrap_or(*terrain);
        result
    };
    let (max_x, max_y) = pattern
        .keys()
        .fold((-1_i64, -1_i64), |(max_x, max_y), Coord { x, y }| {
            (max_x.max(*x), max_y.max(*y))
        });
    let col_result = find_column_reflection(max_x, max_y, &get_terrain)
        .filter(|refl| skip.as_ref().map(|r| !r.eq(refl)).unwrap_or(true))
        .next();

    let result = col_result.or_else(|| {
        find_row_reflection(max_x, max_y, &get_terrain)
            .filter(|refl| skip.as_ref().map(|r| !r.eq(refl)).unwrap_or(true))
            .next()
    });
    result
}

fn find_column_reflection<'a>(
    max_x: i64,
    max_y: i64,
    get_terrain: &'a impl Fn(Coord) -> char,
) -> impl Iterator<Item = Reflection> + 'a {
    (1..=max_x)
        .filter(move |&col| {
            let num_cols = col.min((max_x + 1) - col);
            (col..col + num_cols).all(|x| {
                (0..=max_y).all(|y| {
                    let x_left = col - 1 - (x - col);
                    let x_right = x;
                    get_terrain(Coord::from(x_right, y)).eq(&get_terrain(Coord::from(x_left, y)))
                })
            })
        })
        .map(Reflection::Col)
}

fn find_row_reflection<'a>(
    max_x: i64,
    max_y: i64,
    get_terrain: &'a impl Fn(Coord) -> char,
) -> impl Iterator<Item = Reflection> + 'a {
    (1..=max_y)
        .filter(move |row| {
            let num_rows = *row.min(&((max_y + 1) - *row));
            (*row..row + num_rows).all(|y| {
                (0..=max_x).all(|x| {
                    let y_left = row - 1 - (y - row);
                    let y_right = y;
                    get_terrain(Coord::from(x, y_right)).eq(&get_terrain(Coord::from(x, y_left)))
                })
            })
        })
        .map(Reflection::Row)
}

fn read_patterns(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| assign_coordinates(pattern).collect::<Pattern>())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_test() {
        assert_eq!(405, part1(&read_patterns(SAMPLE_INPUT)).unwrap());
    }

    #[test]
    fn part2_test() {
        assert_eq!(400, part2(&read_patterns(SAMPLE_INPUT)).unwrap());
    }
}
