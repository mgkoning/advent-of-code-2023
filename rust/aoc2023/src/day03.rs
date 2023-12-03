use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> Result<(), String> {
    let schematic = read_schematic(input);
    let part1 = part1(&schematic);
    println!("Part 1: {part1}");
    let part2 = part2(&schematic);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(schematic: &Vec<Entry>) -> u32 {
    let symbol_set: HashSet<Coord> = schematic.iter().filter_map(Entry::to_symbol).collect();
    schematic
        .iter()
        .filter_map(Entry::to_part)
        .filter(|part| adjacent_symbols(&symbol_set, part).next().is_some())
        .map(|part| part.value)
        .sum()
}

fn part2(schematic: &Vec<Entry>) -> u32 {
    let gear_symbols: HashSet<Coord> = schematic
        .iter()
        .filter_map(|e| e.to_symbol_of_kind('*'))
        .collect();
    let mut parts_by_gear = HashMap::<Coord, Vec<u32>>::new();
    schematic
        .iter()
        .filter_map(Entry::to_part)
        .flat_map(|part| adjacent_symbols(&gear_symbols, part).map(|coord| (coord, part.value)))
        .for_each(|(gear, part)| {
            parts_by_gear
                .entry(gear)
                .and_modify(|parts| parts.push(part))
                .or_insert(vec![part]);
        });
    parts_by_gear
        .iter()
        .filter_map(|(_, parts)| match &parts[..] {
            [a, b] => Some(a * b),
            _ => None,
        })
        .sum()
}

fn adjacent_symbols<'a>(
    symbols: &'a HashSet<Coord>,
    part_number: &'a PartNumber,
) -> impl Iterator<Item = Coord> + 'a {
    adjacents(part_number).filter(|a| symbols.contains(&a))
}

fn adjacents<'a>(
    PartNumber {
        start: (x, y), len, ..
    }: &'a PartNumber,
) -> impl Iterator<Item = Coord> + 'a {
    (x - 1..=x + len)
        .flat_map(move |x| [(x, y - 1), (x, y + 1)])
        .chain([(*x - 1, *y), (*x + len, *y)])
}

fn read_schematic(schematic: &str) -> Vec<Entry> {
    schematic
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .fold(ReadState::start(y), ReadState::update)
                .finish()
        })
        .flatten()
        .collect()
}

type Coord = (i32, i32);

#[derive(Debug, PartialEq)]
struct PartNumber {
    start: Coord,
    len: i32,
    value: u32,
}
impl PartNumber {
    fn start(from: Coord, digit: u32) -> PartNumber {
        PartNumber {
            start: from,
            len: 1,
            value: digit,
        }
    }
    fn add_digit(self, digit: u32) -> PartNumber {
        PartNumber {
            len: self.len + 1,
            value: self.value * 10 + digit,
            ..self
        }
    }
}

#[derive(Debug, PartialEq)]
enum Entry {
    Symbol(Coord, char),
    Number(PartNumber),
}
impl Entry {
    fn to_symbol(&self) -> Option<Coord> {
        match self {
            Entry::Symbol(coord, _) => Some(*coord),
            _ => None,
        }
    }
    fn to_symbol_of_kind(&self, kind: char) -> Option<Coord> {
        match self {
            Entry::Symbol(coord, c) if *c == kind => Some(*coord),
            _ => None,
        }
    }
    fn to_part(&self) -> Option<&PartNumber> {
        match self {
            Entry::Number(p) => Some(p),
            _ => None,
        }
    }
}

enum ReadMode {
    Regular,
    Number(PartNumber),
}

struct ReadState {
    mode: ReadMode,
    y: i32,
    entries: Vec<Entry>,
}
impl ReadState {
    fn start(y: usize) -> ReadState {
        ReadState {
            mode: ReadMode::Regular,
            y: y as i32,
            entries: vec![],
        }
    }
    fn update(self, (x, c): (usize, char)) -> ReadState {
        match (self.mode, c.to_digit(10)) {
            (ReadMode::Regular, Some(d)) => ReadState {
                mode: ReadMode::Number(PartNumber::start((x as i32, self.y), d)),
                ..self
            },
            (ReadMode::Number(part), Some(d)) => ReadState {
                mode: ReadMode::Number(part.add_digit(d)),
                ..self
            },
            (mode, None) => {
                let mut entries = self.entries;
                if let ReadMode::Number(part) = mode {
                    entries.push(Entry::Number(part));
                }
                if c != '.' {
                    entries.push(Entry::Symbol((x as i32, self.y), c))
                }
                ReadState {
                    mode: ReadMode::Regular,
                    entries,
                    ..self
                }
            }
        }
    }
    fn finish(self) -> Vec<Entry> {
        let mut entries = self.entries;
        if let ReadMode::Number(part) = self.mode {
            entries.push(Entry::Number(part));
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn read_schematic_test() {
        assert_eq!(
            read_schematic("35..42"),
            vec![
                Entry::Number(PartNumber {
                    start: (0, 0),
                    len: 2,
                    value: 35
                }),
                Entry::Number(PartNumber {
                    start: (4, 0),
                    len: 2,
                    value: 42
                })
            ]
        )
    }

    #[test]
    fn part1_test() {
        assert_eq!(4361, part1(&read_schematic(TEST_INPUT)));
    }

    #[test]
    fn part2_test() {
        assert_eq!(467835, part2(&read_schematic(TEST_INPUT)));
    }

    #[test]
    fn adjacents_test() {
        assert_eq!(
            adjacents(&PartNumber {
                start: (0, 0),
                len: 3,
                value: 467
            })
            .collect::<Vec<_>>(),
            vec![
                (-1, -1),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 1),
                (2, -1),
                (2, 1),
                (3, -1),
                (3, 1),
                (-1, 0),
                (3, 0)
            ]
        )
    }
}
