use crate::util::read;

pub fn run(input: &str) -> Result<(), String> {
    let races = read_races(input)?;
    let part1 = part1(&races);
    println!("Part 1: {part1}");
    let amended_race = read_amended(input)?;
    let part2 = ways_to_win(&amended_race);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(races: &Vec<(i64, i64)>) -> i64 {
    races.iter().map(ways_to_win).product()
}

fn ways_to_win((time, record): &(i64, i64)) -> i64 {
    (0..=*time)
        .map(|wait| (time - wait) * wait)
        .filter(|d| record < d)
        .count() as i64
}

fn read_amended(input: &str) -> Result<(i64, i64), String> {
    let x = input
        .replace(" ", "")
        .lines()
        .map(|line| line.split_once(":").map(|(_, v)| read::<i64>(v)).unwrap())
        .collect::<Result<Vec<i64>, String>>()?;
    match &x[..] {
        [time, distance] => Ok((*time, *distance)),
        _ => Err("Problem reading input".to_owned()),
    }
}

fn read_races(input: &str) -> Result<Vec<(i64, i64)>, String> {
    let times_distances = input
        .lines()
        .map(read_line)
        .collect::<Result<Vec<_>, String>>()?;
    match &times_distances[..] {
        [times, distances] => Ok(times.iter().zip(distances).map(|(&a, &b)| (a, b)).collect()),
        _ => Err("Problem reading input".to_owned()),
    }
}

fn read_line(input: &str) -> Result<Vec<i64>, String> {
    input.split_whitespace().skip(1).map(read::<i64>).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_test() {
        assert_eq!(288, part1(&read_races(SAMPLE_INPUT).unwrap()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(71503, ways_to_win(&read_amended(SAMPLE_INPUT).unwrap()));
    }
}
