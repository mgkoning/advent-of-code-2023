use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn origin() -> Coord {
        Coord { x: 0, y: 0 }
    }
    pub fn from(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }
    pub fn north(&self) -> Coord {
        Coord::from(self.x, self.y - 1)
    }
    pub fn south(&self) -> Coord {
        Coord::from(self.x, self.y + 1)
    }
    pub fn east(&self) -> Coord {
        Coord::from(self.x + 1, self.y)
    }
    pub fn west(&self) -> Coord {
        Coord::from(self.x - 1, self.y)
    }
    pub fn move_by(&self, (dx, dy): (i64, i64)) -> Coord {
        Coord::from(self.x + dx, self.y + dy)
    }
    pub fn go(&self, direction: Direction) -> Coord {
        match direction {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }
    pub fn go_by(&self, direction: Direction, length: i64) -> Coord {
        match direction {
            Direction::North => self.move_by((0, -length)),
            Direction::East => self.move_by((length, 0)),
            Direction::South => self.move_by((0, length)),
            Direction::West => self.move_by((-length, 0)),
        }
    }
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    pub fn neighbors4(&self) -> [Coord; 4] {
        [self.north(), self.east(), self.south(), self.west()]
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq)]
pub struct Step {
    pub at: Coord,
    pub distance: i64,
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    North,
    West,
    South,
    East,
}
impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Self::North => Direction::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
    pub fn turn_left(&self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

pub fn read<T: FromStr>(value: &str) -> Result<T, String>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    value
        .parse::<T>()
        .map_err(|e| format!("Failed to read {value}: {e}"))
}

/** Splits the provided string on whitespace and parses all parts as T. */
pub fn read_all<T: FromStr>(value: &str) -> Result<Vec<T>, String>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    value.split_whitespace().map(read::<T>).try_collect()
}

pub fn read_all_sep_by<T: FromStr>(value: &str, separator: char) -> Result<Vec<T>, String>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    value
        .split(separator)
        .filter(|p| !p.is_empty())
        .map(read::<T>)
        .try_collect()
}

pub fn assign_coordinates(spec: &'_ str) -> impl Iterator<Item = (Coord, char)> + '_ {
    spec.lines().enumerate().flat_map(|(y, l)| {
        l.chars()
            .enumerate()
            .map(move |(x, c)| (Coord::from(x as i64, y as i64), c))
    })
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) if a < b => gcd(a, b.rem_euclid(a)),
        (a, b) if b < a => gcd(b, a.rem_euclid(b)),
        _ => panic!("never finished gcd"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_test() {
        assert_eq!(15, lcm(3, 5))
    }

    #[test]
    fn gcd_test() {
        assert_eq!(6, gcd(48, 18))
    }
}
