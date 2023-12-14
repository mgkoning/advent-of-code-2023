use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
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
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
