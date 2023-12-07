use std::collections::HashMap;

use itertools::Itertools;

use crate::util::read;

struct Hand {
    cards: Vec<char>,
    rank: i64,
}

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy)]
struct HandSorting {
    value: i64,
    tie_break_value: i64,
}
impl Ord for HandSorting {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .cmp(&other.value)
            .then(self.tie_break_value.cmp(&other.tie_break_value))
    }
}

pub fn run(input: &str) -> Result<(), String> {
    let hands = read_input(input)?;
    let part1 = part1(&hands);
    println!("Part 1: {part1}");
    let part2 = part2(&hands);
    println!("Part 2: {part2}");
    Ok(())
}

fn part1(hands: &Vec<Hand>) -> i64 {
    let card_values: HashMap<char, i64> =
        "23456789TJQKA".chars().zip(0..).collect::<HashMap<_, _>>();
    winnings(hands, hand_value_p1, &card_values)
}

fn part2(hands: &Vec<Hand>) -> i64 {
    let card_values: HashMap<char, i64> =
        "J23456789TQKA".chars().zip(0..).collect::<HashMap<_, _>>();
    winnings(hands, hand_value_p2, &card_values)
}

fn winnings(
    hands: &Vec<Hand>,
    hand_value: impl Fn(&Vec<char>) -> i64,
    card_values: &HashMap<char, i64>,
) -> i64 {
    hands
        .iter()
        .map(|h| {
            let sorting = HandSorting {
                value: hand_value(&h.cards),
                tie_break_value: sort_value(&h.cards, card_values),
            };
            (h, sorting)
        })
        .sorted_by_key(|(_, s)| *s)
        .zip(1..)
        .map(|((h, _), i)| h.rank * i)
        .sum()
}

fn sort_value(cards: &Vec<char>, card_values: &HashMap<char, i64>) -> i64 {
    // interpret the cards as a base 13 number based on a (provided) 0..13 mapping of their labels
    cards.iter().fold(0, |acc, c| acc * 13 + card_values[c])
}

fn hand_value_p1(cards: &Vec<char>) -> i64 {
    // count the cards by label to determine what type the hand is
    let count_map = cards.into_iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    });
    let mut counts = count_map.values().collect::<Vec<_>>();
    // sort the counts so the match can be simpler
    counts.sort_unstable();
    match &counts[..] {
        [5] => 6,          // five of a kind
        [1, 4] => 5,       // four of a kind
        [2, 3] => 4,       // full house
        [1, 1, 3] => 3,    // three of a kind
        [1, 2, 2] => 2,    // two pair
        [1, 1, 1, 2] => 1, // one pair
        _ => 0,            // high card
    }
}

const ALL_CARDS: [char; 12] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
fn hand_value_p2(cards: &Vec<char>) -> i64 {
    let (jokers, others): (Vec<&char>, Vec<&char>) = cards.iter().partition(|&c| *c == 'J');
    if jokers.is_empty() {
        hand_value_p1(cards)
    } else {
        // if there are any jokers, just "brute force" every replacement option and see what yields the highest type.
        // not very elegant, but fast enough.
        let options = jokers
            .iter()
            .map(|_| &ALL_CARDS)
            .multi_cartesian_product()
            .collect_vec();
        options
            .iter()
            .map(|o| {
                hand_value_p1(
                    &o.iter()
                        .cloned()
                        .chain(others.iter().cloned())
                        .cloned()
                        .collect_vec(),
                )
            })
            .max()
            .unwrap_or_default()
    }
}

fn read_input(input: &str) -> Result<Vec<Hand>, String> {
    input.lines().map(read_hand).collect()
}

fn read_hand(input: &str) -> Result<Hand, String> {
    match input.split_once(" ") {
        Some((cards, rank)) => Ok(Hand {
            cards: cards.chars().collect::<Vec<_>>(),
            rank: read::<i64>(rank)?,
        }),
        _ => Err(format!("Could not read hand from {input}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn hand_value_test() {
        assert_eq!(6, hand_value_p1(&"KKKKK".chars().collect()));
        assert_eq!(4, hand_value_p1(&"KKQQK".chars().collect()));
    }

    #[test]
    fn part1_test() {
        assert_eq!(6440, part1(&read_input(SAMPLE_INPUT).unwrap()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(5905, part2(&read_input(SAMPLE_INPUT).unwrap()));
    }
}
