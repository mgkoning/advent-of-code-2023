use itertools::Itertools;

use crate::util::read_all;

pub fn run(input: &str) -> Result<(), String> {
    let readings = read_input(input)?;
    let part1 = part1(&readings);
    println!("Part 1: {part1}");
    let part2 = part2(&readings);
    println!("Part 2: {part2}");
    Ok(())
}

fn read_input(input: &str) -> Result<Vec<Vec<i64>>, String> {
    input.lines().map(read_all::<i64>).try_collect()
}

fn part1(readings: &Vec<Vec<i64>>) -> i64 {
    readings.iter().map(determine_next).sum()
}

fn determine_next(reading: &Vec<i64>) -> i64 {
    iterate_differences(&reading, |mut v| v.pop().unwrap()).sum()
}

fn iterate_differences(
    reading: &Vec<i64>,
    extract: impl Fn(Vec<i64>) -> i64,
) -> impl Iterator<Item = i64> {
    std::iter::successors(Some(reading.clone()), |prev| {
        Some(prev.iter().map_windows(|[&a, &b]| b - a).collect_vec())
    })
    .take_while(|v| v.iter().any(|&i| i != 0))
    .map(extract)
}

fn part2(readings: &Vec<Vec<i64>>) -> i64 {
    readings.iter().map(determine_prev).sum()
}

fn determine_prev(reading: &Vec<i64>) -> i64 {
    iterate_differences(&reading, |v| v[0])
        .collect_vec()
        .iter()
        .rfold(0, |acc, next| next - acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_test() {
        let readings: Vec<_> = read_input(SAMPLE_INPUT).unwrap();
        assert_eq!(114, part1(&readings));
    }

    #[test]
    fn part2_test() {
        let readings: Vec<_> = read_input(SAMPLE_INPUT).unwrap();
        assert_eq!(2, part2(&readings));
    }
}
