use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::util::{assign_coordinates, Coord, Step};

pub fn run(input: &str) -> Result<(), String> {
    let (maze, start) = read_input(input)?;
    let distances = distances(start, &maze);
    let part1 = part1(&distances)?;
    println!("Part 1: {part1}");
    let part2 = part2(&maze, &distances);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(distances: &HashMap<Coord, i64>) -> Result<i64, String> {
    distances
        .values()
        .max()
        .ok_or_else(|| "Distances is empty".to_owned())
        .copied()
}

/**
We can consider the loop a convoluted polygon. To find what points are inside the polygon, we can
use the [ray casting algorithm]: draw a line from "outside" to each point that is not in the loop.
Count the number of time it crosses a pipe. If the number is odd, the point is inside the loop.
To simplify, we always cast from x = 0 and ignore loop parts that are '-', '7' and 'F', as we
assume the ray can "squeeze" past those, i.e., the ray follows the upper part of a tile.
It also works if you discount '-', 'J' and 'L' as "passable".

It could be made more efficient by choosing the shortest path to "outside", in which case also
the y direction should be considered. Another optimization would be to determine the smallest
bounding rectangle to use for "outside" coordinates. I've left these optimizations out since it
currently runs takes ~50ms in release mode on my machine.

[ray casting algorithm]: https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
*/
fn part2(maze: &HashMap<Coord, char>, distances: &HashMap<Coord, i64>) -> i64 {
    let can_pass = HashSet::from(['-', '7', 'F']);
    let is_inside = |candidate: &Coord| {
        let crossings = (0..candidate.x)
            .map(|x| Coord::from(x, candidate.y))
            .filter(|c| {
                distances.contains_key(c) && maze.get(c).map(|p| !can_pass.contains(p)).unwrap()
            })
            .count();
        crossings % 2 == 1
    };
    maze.keys()
        .filter(|c| !distances.contains_key(c) && is_inside(c))
        .count() as i64
}

fn distances(start: Coord, maze: &HashMap<Coord, char>) -> HashMap<Coord, i64> {
    // I used Dijkstra's algorithm for this, but it wasn't really necessary in the end.
    fn all_paths(
        mut to_visit: BinaryHeap<Step>,
        mut seen: HashSet<Coord>,
        maze: &HashMap<Coord, char>,
        mut result: HashMap<Coord, i64>,
    ) -> HashMap<Coord, i64> {
        match to_visit.pop() {
            None => result,
            Some(Step { at, distance }) => {
                result.insert(at, distance);
                neighbors(&at, maze.get(&at).unwrap())
                    .iter()
                    .filter(|&n| seen.insert(*n))
                    .map(|n| Step {
                        at: *n,
                        distance: distance + 1,
                    })
                    .for_each(|s| to_visit.push(s));
                all_paths(to_visit, seen, maze, result)
            }
        }
    }
    let start_step = Step {
        at: start,
        distance: 0,
    };
    all_paths(
        BinaryHeap::from([start_step]),
        HashSet::from([start]),
        maze,
        HashMap::new(),
    )
}

fn neighbors(at: &Coord, pipe: &char) -> [Coord; 2] {
    match pipe {
        '|' => [at.north(), at.south()],
        '-' => [at.east(), at.west()],
        'L' => [at.north(), at.east()],
        'J' => [at.north(), at.west()],
        '7' => [at.south(), at.west()],
        'F' => [at.south(), at.east()],
        _ => panic!("Unknown pipe type {pipe}"),
    }
}

fn read_input(input: &str) -> Result<(HashMap<Coord, char>, Coord), String> {
    let mut maze = assign_coordinates(input).collect::<HashMap<_, _>>();
    let start = maze
        .iter()
        .find_map(|(k, &v)| if v == 'S' { Some(k) } else { None })
        .ok_or_else(|| "Start not found".to_owned())
        .copied()?;
    let start_pipe = determine_start_pipe(&maze, start)?;
    maze.entry(start).and_modify(|c| *c = start_pipe);
    Ok((maze, start))
}

fn determine_start_pipe(maze: &HashMap<Coord, char>, start: Coord) -> Result<char, String> {
    let is_neighbor = |coord| {
        maze.get(coord)
            .filter(|&p| *p != '.')
            .map(|p| neighbors(coord, p).contains(&start))
            .unwrap_or(false)
    };
    match (
        is_neighbor(&start.north()),
        is_neighbor(&start.south()),
        is_neighbor(&start.east()),
        is_neighbor(&start.west()),
    ) {
        (true, true, _, _) => Some('|'),
        (_, _, true, true) => Some('-'),
        (true, _, true, _) => Some('L'),
        (true, _, _, true) => Some('J'),
        (_, true, _, true) => Some('7'),
        (_, true, true, _) => Some('F'),
        _ => None,
    }
    .ok_or_else(|| format!("Could not determine start pipe"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_P2_1: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const SAMPLE_INPUT_P2_2: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part2_sample1_test() {
        let (maze, start) = read_input(SAMPLE_INPUT_P2_1).unwrap();
        let distances = distances(start, &maze);
        assert_eq!(4, part2(&maze, &distances));
    }

    #[test]
    fn part2_sample2_test() {
        let (maze, start) = read_input(SAMPLE_INPUT_P2_2).unwrap();
        let distances = distances(start, &maze);
        assert_eq!(10, part2(&maze, &distances));
    }
}
