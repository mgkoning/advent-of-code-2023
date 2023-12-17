use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

use crate::util::{assign_coordinates, Coord, Direction};

#[derive(PartialEq, Eq, Debug)]
struct Step {
    at: Coord,
    distance: u32,
    direction: Direction,
    straight_for: u32,
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

pub fn run(input: &str) -> Result<(), String> {
    let city = read_city(input)?;
    let part1 = part1(&city)?;
    println!("Part 1: {part1}");
    let part2 = part2(&city)?;
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(city: &HashMap<Coord, u32>) -> Result<u32, String> {
    let (max_x, max_y) = city
        .keys()
        .fold((0, 0), |(mx, my), Coord { x, y }| (mx.max(*x), my.max(*y)));
    let target = Coord::from(max_x, max_y);
    let neighbors = |s: Step| neighbors(s, 3, 0, city);
    distance(Direction::South, target, neighbors, |_| true).ok_or("No path found".to_string())
}

fn part2(city: &HashMap<Coord, u32>) -> Result<u32, String> {
    let (max_x, max_y) = city
        .keys()
        .fold((0, 0), |(mx, my), Coord { x, y }| (mx.max(*x), my.max(*y)));
    let target = Coord::from(max_x, max_y);
    let can_stop = |s: &Step| 3 < s.straight_for;
    let neighbors_ultra = |s: Step| neighbors(s, 10, 4, city);
    [Direction::South, Direction::East]
        .into_iter()
        .map(|start_dir| distance(start_dir, target, neighbors_ultra, can_stop))
        .filter_map(|s| s)
        .min()
        .ok_or("No path found".to_string())
}

fn distance(
    start_direction: Direction,
    target: Coord,
    get_neighbors: impl Fn(Step) -> Vec<Step>,
    can_stop: impl Fn(&Step) -> bool,
) -> Option<u32> {
    let start_step = Step {
        at: Coord::origin(),
        distance: 0,
        direction: start_direction,
        straight_for: 0,
    };
    let to_key = |s: &Step| (s.at, s.direction, s.straight_for);
    let mut seen = HashSet::from([to_key(&start_step)]);
    let mut to_visit = BinaryHeap::from([start_step]);
    loop {
        match to_visit.pop() {
            None => return None,
            Some(step @ Step { at, distance, .. }) => {
                if at.eq(&target) && can_stop(&step) {
                    return Some(distance);
                }
                get_neighbors(step)
                    .into_iter()
                    .filter(|n| seen.insert(to_key(&n)))
                    .for_each(|s| to_visit.push(s));
            }
        }
    }
}

fn neighbors(
    Step {
        at,
        distance,
        direction,
        straight_for,
        ..
    }: Step,
    max_straight: u32,
    min_straight_for_turn: u32,
    city: &HashMap<Coord, u32>,
) -> Vec<Step> {
    let turn_allowed = min_straight_for_turn <= straight_for;
    [
        (direction, straight_for + 1, straight_for < max_straight),
        (direction.turn_left(), 1, turn_allowed),
        (direction.turn_right(), 1, turn_allowed),
    ]
    .into_iter()
    .filter(|(_, _, allowed)| *allowed)
    .map(|(d, s, _)| (d, s, at.go(d)))
    .filter_map(|(d, s, pos)| {
        city.get(&pos).map(|dis| Step {
            at: pos,
            distance: distance + dis,
            direction: d,
            straight_for: s,
        })
    })
    .collect_vec()
}

fn read_city(input: &str) -> Result<HashMap<Coord, u32>, String> {
    assign_coordinates(input)
        .map(|(coord, v)| {
            v.to_digit(10)
                .map(|d| (coord, d))
                .ok_or_else(|| format!("Could not read {v}"))
        })
        .collect::<Result<HashMap<Coord, u32>, String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const UNFORTUNATE: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part1_test() {
        assert_eq!(102, part1(&read_city(SAMPLE_INPUT).unwrap()).unwrap());
    }

    #[test]
    fn part2_test() {
        assert_eq!(94, part2(&read_city(SAMPLE_INPUT).unwrap()).unwrap());
        assert_eq!(71, part2(&read_city(UNFORTUNATE).unwrap()).unwrap());
    }
}
