use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn run(input: &str) -> Result<(), String> {
    let (instr, map) = read_input(input)?;
    let part1 = part1(&instr, &map);
    println!("Part 1: {part1}");
    let part2 = part2(&instr, &map);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(instr: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> i64 {
    follow_map("AAA", &instr, |pos| "ZZZ".eq(pos), &map)
}

fn part2(instr: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> i64 {
    let start = map
        .keys()
        .filter(|p| p.ends_with('A'))
        .cloned()
        .collect_vec();
    start
        .iter()
        .map(|from| follow_map(from, instr, |p| p.ends_with('Z'), map))
        .reduce(lcm)
        .unwrap()
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) if a < b => gcd(a, b.rem_euclid(a)),
        (a, b) if b < a => gcd(b, a.rem_euclid(b)),
        _ => panic!("never finished gcd"),
    }
}

fn follow_map(
    from: &str,
    instr: &Vec<char>,
    until: fn(&str) -> bool,
    map: &HashMap<&str, (&str, &str)>,
) -> i64 {
    instr
        .iter()
        .cycle()
        .scan(from, |state, next| {
            next_path(state, next, &map).map(|n| {
                *state = n;
                n
            })
        })
        .zip(1..)
        .skip_while(|(pos, _)| !until(&pos))
        .next()
        .unwrap()
        .1
}

fn next_path<'a>(
    from: &str,
    instr: &char,
    map: &HashMap<&str, (&'a str, &'a str)>,
) -> Option<&'a str> {
    match (instr, map.get(from)) {
        ('L', Some((left, _))) => Some(left),
        ('R', Some((_, right))) => Some(right),
        _ => None,
    }
}

fn read_input(input: &str) -> Result<(Vec<char>, HashMap<&str, (&str, &str)>), String> {
    match input.split_once("\n\n") {
        Some((instr, map)) => Ok((instr.chars().collect_vec(), read_map(map)?)),
        None => Err("Could not read input".to_owned()),
    }
}

fn read_map(map_input: &str) -> Result<HashMap<&str, (&str, &str)>, String> {
    let line_re = Regex::new(r"(?m)^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$").unwrap();
    Ok(line_re
        .captures_iter(map_input)
        .map(|c| c.extract())
        .map(|(_, [from, left, right])| (from, (left, right)))
        .collect::<HashMap<_, _>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_INPUT_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn read_input_test() {
        let (instr, map) = read_input(SAMPLE_INPUT).unwrap();
        assert_eq!(vec!['L', 'L', 'R'], instr);
        assert_eq!(
            HashMap::from([
                ("AAA", ("BBB", "BBB")),
                ("BBB", ("AAA", "ZZZ")),
                ("ZZZ", ("ZZZ", "ZZZ"))
            ]),
            map
        );
    }

    #[test]
    fn part1_test() {
        let (instr, map) = read_input(SAMPLE_INPUT).unwrap();
        assert_eq!(6, part1(&instr, &map));
    }

    #[test]
    fn part2_test() {
        let (instr, map) = read_input(SAMPLE_INPUT_2).unwrap();
        assert_eq!(6, part2(&instr, &map));
    }
}
