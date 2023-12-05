pub fn run(input: &str) -> Result<(), String> {
    let (seeds, maps) = read_input(input)?;
    let part1 = part1(&seeds, &maps)?;
    println!("Part 1: {part1}");
    Ok(())
}

fn part1(seeds: &Vec<u64>, maps: &Vec<Map>) -> Result<u64, String> {
    seeds
        .iter()
        .map(|s| run_conversions(*s, maps))
        .min()
        .ok_or("No result found".to_owned())
}

fn run_conversions(value: u64, maps: &Vec<Map>) -> u64 {
    maps.iter().fold(value, |acc, map| map.convert(acc))
}

fn read_input(input: &str) -> Result<(Vec<u64>, Vec<Map>), String> {
    match &input.split("\n\n").collect::<Vec<_>>()[..] {
        [seeds, maps @ ..] => Ok((
            read_seeds(seeds)?,
            maps.iter()
                .map(|&map| read_map(map))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        _ => Err("Could not find seeds and maps".to_owned()),
    }
}

fn read_seeds(line: &str) -> Result<Vec<u64>, String> {
    match line.split_once(": ") {
        None => Err("Could not read seeds".to_owned()),
        Some((_, s)) => {
            let seeds = s
                .split_whitespace()
                .map(|n| read_u64(n))
                .collect::<Result<Vec<u64>, _>>()?;
            Ok(seeds)
        }
    }
}

fn read_map(map: &str) -> Result<Map, String> {
    match &map.lines().collect::<Vec<_>>()[..] {
        [name, conversions @ ..] => Ok(Map {
            name: (*name).to_owned(),
            conversions: conversions
                .iter()
                .map(|&c| read_conversion(c))
                .collect::<Result<Vec<_>, _>>()?,
        }),

        _ => Err("Could not read map".to_owned()),
    }
}

fn read_conversion(conversion: &str) -> Result<Conversion, String> {
    match &conversion.split_whitespace().collect::<Vec<_>>()[..] {
        [destination, source, length] => Ok(Conversion {
            destination_start: read_u64(&destination)?,
            source_start: read_u64(&source)?,
            length: read_u64(&length)?,
        }),
        _ => Err(format!("Could not read conversion '{conversion}'")),
    }
}

fn read_u64(value: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|e| format!("Error reading {value}: {e}"))
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    name: String,
    conversions: Vec<Conversion>,
}
impl Map {
    fn convert(&self, value: u64) -> u64 {
        self.conversions
            .iter()
            .find_map(|c| {
                if c.source_start <= value && value < c.source_start + c.length {
                    let offset = value - c.source_start;
                    Some(c.destination_start + offset)
                } else {
                    None
                }
            })
            .unwrap_or(value)
    }
}

#[derive(Debug)]
struct Conversion {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn part1_test() {
        let (s, m) = read_input(SAMPLE_INPUT).unwrap();
        assert_eq!(35, part1(&s, &m).unwrap());
    }
}
