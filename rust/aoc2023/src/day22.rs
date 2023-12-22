use std::{collections::VecDeque, iter::repeat};

use itertools::Itertools;
use regex::Regex;

use crate::util::read;

#[derive(Debug, Clone)]
struct Brick {
    from: Coord3,
    to: Coord3,
}
impl Brick {
    fn drop_to(&self, new_z: i64) -> Brick {
        Brick {
            from: Coord3 {
                z: new_z,
                ..self.from
            },
            to: Coord3 {
                z: self.to.z - (self.from.z - new_z),
                ..self.to
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64,
}

pub fn run(input: &str) -> Result<(), String> {
    let bricks = read_bricks(input)?;
    check_bricks(&bricks);
    let part1 = part1(&bricks);
    println!("Part 1: {part1}");
    let part2 = part2(&bricks);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(bricks: &[Brick]) -> usize {
    let dropped = drop_bricks(&bricks);
    let supporting = determine_supports(&dropped);
    (0..dropped.len())
        .filter(|i| {
            let supported_by = supporting
                .iter()
                .filter(|(from, _)| from == i)
                .map(|(_, to)| to)
                .collect_vec();
            supported_by.iter().all(|&b| {
                let other_supports = supporting
                    .iter()
                    .filter(|&(from, to)| to == b && from != i)
                    .count();
                other_supports > 0
            })
        })
        .count()
}

fn part2(bricks: &[Brick]) -> usize {
    let stacked = drop_bricks(&bricks);
    let node_count = stacked.len();
    let edges = determine_supports(&stacked);
    score_part2(edges, node_count)
}

fn score_part2(edges: Vec<(usize, usize)>, node_count: usize) -> usize {
    let mut reachable = (0..node_count)
        .map(|_| repeat(false).take(node_count).collect_vec())
        .collect_vec();
    let leaves = (0..node_count)
        .filter(|n| edges.iter().filter(|(from, _)| from == n).count() == 0)
        .collect_vec();
    let mut to_follow = VecDeque::from_iter(edges.iter().filter(|(_, to)| leaves.contains(to)));
    while let Some((a, b)) = to_follow.pop_front() {
        reachable[*a][*b] = true;
        (0..node_count)
            .for_each(|other| reachable[*a][other] = reachable[*a][other] || reachable[*b][other]);
        edges
            .iter()
            .filter(|(_, to)| to == a)
            .for_each(|e| to_follow.push_back(e));
    }
    // Reachability calculation is slow because of repeatedly considering an edge. Use topological sort
    // to improve?
    println!("Reachability calculated");

    (0..node_count)
        .map(|n| {
            let score = (0..node_count)
                .filter(|o| {
                    *o != n
                        && reachable[n][*o]
                        && (0..node_count)
                            .filter(|p| *p != n && *p != *o && reachable[*p][*o])
                            .all(|p| reachable[n][p] || reachable[p][n])
                })
                .count();
            score
        })
        .sum()
}

fn determine_supports(dropped: &Vec<Brick>) -> Vec<(usize, usize)> {
    dropped
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            dropped
                .iter()
                .enumerate()
                .filter(|(_, c)| c.from.z == b.to.z + 1 && are_horizontally_intersected(b, c))
                .map(move |(j, _)| (i, j))
        })
        .collect_vec()
}

fn drop_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let mut sorted_bricks = bricks
        .iter()
        .sorted_by(|a, b| a.from.z.cmp(&b.from.z))
        .collect::<VecDeque<_>>();
    let mut stacked_bricks = Vec::new();
    while let Some(b) = sorted_bricks.pop_front() {
        if b.from.z == 1 {
            stacked_bricks.push(b.clone());
            continue;
        }
        let drop_to = stacked_bricks
            .iter()
            .rev()
            .filter(|s| are_horizontally_intersected(s, b))
            .map(|s| s.to.z + 1)
            .max()
            .unwrap_or(1);
        stacked_bricks.push(b.drop_to(drop_to));
    }
    stacked_bricks
}

fn are_horizontally_intersected(a: &Brick, b: &Brick) -> bool {
    a.from.x <= b.to.x && b.from.x <= a.to.x && a.from.y <= b.to.y && b.from.y <= a.to.y
}

/// Check that assumptions about the bricks are OK
fn check_bricks(bricks: &[Brick]) {
    bricks
        .iter()
        .map(|b| [b.to.x - b.from.x, b.to.y - b.from.y, b.to.z - b.from.z])
        .enumerate()
        .for_each(|(i, diffs @ [dx, dy, dz])| {
            if diffs.iter().filter(|d| **d > 0).count() > 1 {
                println!("Brick {i} has multiple coordinate changes: {dx} {dy} {dz}");
            }
            if diffs.iter().filter(|d| **d < 0).count() > 0 {
                println!("Brick {i} has a negative coordinate change: {dx} {dy} {dz}");
            }
        });
}

fn read_bricks(input: &str) -> Result<Vec<Brick>, String> {
    let brick_re = Regex::new(r"(?m)^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    brick_re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x0, y0, z0, x1, y1, z1])| {
            let from = Coord3 {
                x: read(x0)?,
                y: read(y0)?,
                z: read(z0)?,
            };
            let to = Coord3 {
                x: read(x1)?,
                y: read(y1)?,
                z: read(z1)?,
            };
            Ok(Brick { from, to })
        })
        .try_collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn are_horizontally_intersected_test() {
        let bricks = read_bricks(SAMPLE_INPUT).unwrap();
        assert_eq!(true, are_horizontally_intersected(&bricks[0], &bricks[1]))
    }

    #[test]
    fn part1_test() {
        let bricks = read_bricks(SAMPLE_INPUT).unwrap();
        assert_eq!(5, part1(&bricks));
    }

    #[test]
    fn part2_test() {
        let bricks = read_bricks(SAMPLE_INPUT).unwrap();
        assert_eq!(7, part2(&bricks));
    }

    #[test]
    fn score_part2_test() {
        let edges = vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (3, 5),
            (4, 5),
            (4, 7),
            (5, 6),
            (8, 7),
        ];
        assert_eq!(7, score_part2(edges, 9));
    }
}
