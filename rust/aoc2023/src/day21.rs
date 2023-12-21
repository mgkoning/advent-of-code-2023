use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::util::{assign_coordinates, Coord, Step};

pub fn run(input: &str) -> Result<(), String> {
    let (rocks, start) = read_garden(input);
    let part1 = part1(&rocks, start);
    println!("Part 1: {part1}");
    Ok(())
}

fn part1(rocks: &HashSet<Coord>, start: Coord) -> usize {
    let start_step = Step {
        at: start,
        distance: 0,
    };
    let mut to_visit = BinaryHeap::from([start_step]);
    let mut seen = HashSet::from([start]);
    let mut result = HashMap::<Coord, i64>::new();
    while let Some(Step { at, distance }) = to_visit.pop() {
        result.insert(at, distance);
        at.neighbors4()
            .iter()
            .filter(|n| !rocks.contains(n))
            .filter(|&n| seen.insert(*n))
            .map(|n| Step {
                at: *n,
                distance: distance + 1,
            })
            .filter(|s| s.distance <= 64)
            .for_each(|s| to_visit.push(s));
    }
    result.values().filter(|&v| *v % 2 == 0).count()
}

fn read_garden(input: &str) -> (HashSet<Coord>, Coord) {
    let mut garden = assign_coordinates(input)
        .filter(|(_, c)| *c != '.')
        .collect::<HashMap<_, _>>();
    let start = *garden.iter().find(|(_, c)| **c == 'S').unwrap().0;
    garden.remove(&start);
    (garden.into_keys().collect::<HashSet<_>>(), start)
}
