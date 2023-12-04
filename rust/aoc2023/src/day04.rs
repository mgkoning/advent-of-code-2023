use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> Result<(), String> {
    let cards = read_cards(input)?;
    let wins_by_card = wins_by_card(&cards);
    let part1: u32 = wins_by_card.values().map(score).sum();
    println!("Part 1: {part1}");
    let part2 = part2(&wins_by_card);
    println!("Part 2: {part2}");
    Ok(())
}

fn score(matches: &u32) -> u32 {
    if *matches < 1 {
        0
    } else {
        2u32.pow(matches - 1)
    }
}

fn part2(wins_by_card: &HashMap<u32, u32>) -> u32 {
    let mut card_copies = wins_by_card
        .keys()
        .map(|&c| (c, 1u32))
        .collect::<HashMap<_, _>>();
    let mut all_cards = wins_by_card.keys().collect::<Vec<_>>();
    all_cards.sort();
    for current in all_cards {
        let wins = wins_by_card.get(current).unwrap_or(&0);
        let num = *card_copies.get(current).unwrap_or(&0);
        for copy in current + 1..=current + wins {
            card_copies
                .entry(copy)
                .and_modify(|n| {
                    *n += num;
                })
                .or_insert(num);
        }
    }
    card_copies.values().sum()
}

fn wins_by_card(cards: &Vec<Card>) -> HashMap<u32, u32> {
    (1u32..)
        .zip(
            cards
                .iter()
                .map(|card| card.found.intersection(&card.winning).count() as u32),
        )
        .collect()
}

fn read_cards(input: &str) -> Result<Vec<Card>, String> {
    input.lines().map(read_card).collect()
}

fn read_card(line: &str) -> Result<Card, String> {
    fn read_columns(value: &str) -> Result<HashSet<u32>, String> {
        value
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .map_err(|e| format!("could not parse {n}: {e}"))
            })
            .collect::<Result<HashSet<u32>, String>>()
    }
    let (_, card) = line.split_once(": ").ok_or("could not find number")?;
    let (winning_col, found_col) = card.split_once(" | ").ok_or("could not find columns")?;
    Ok(Card {
        winning: read_columns(winning_col)?,
        found: read_columns(found_col)?,
    })
}

struct Card {
    winning: HashSet<u32>,
    found: HashSet<u32>,
}
