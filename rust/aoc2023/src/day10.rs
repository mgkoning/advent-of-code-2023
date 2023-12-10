use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::util::{assign_coordinates, Coord};

pub fn run(input: &str) -> Result<(), String> {
    let (maze, start) = read_input(input)?;
    let part1 = part1(start, &maze)?;
    println!("Part 1: {part1}");
    Ok(())
}

fn part1(start: Coord, maze: &HashMap<Coord, char>) -> Result<i64, String> {
    distances(start, maze)
        .values()
        .max()
        .ok_or_else(|| "Distances is empty".to_owned())
        .copied()
}

#[derive(PartialEq, Eq)]
struct Step {
    at: Coord,
    distance: i64,
}
impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn distances(start: Coord, maze: &HashMap<Coord, char>) -> HashMap<Coord, i64> {
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
    let mut maze = assign_coordinates(input)
        .filter(|(_, c)| *c != '.')
        .collect::<HashMap<_, _>>();
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
