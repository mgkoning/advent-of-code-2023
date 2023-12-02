use std::collections::HashMap;

pub fn run(input: &str) -> Result<(), String> {
    let games = read_games(input)?;
    let part1 = part_1(&games);
    println!("Part 1: {part1}");
    let part2 = part_2(&games);
    println!("Part 2: {part2}");
    Ok(())
}

fn part_1(games: &Vec<Game>) -> u64 {
    let game_size = GameSize {
        red: 12,
        green: 13,
        blue: 14,
    };
    fn is_possible(size: &GameSize, game: &Game) -> bool {
        game.draws
            .iter()
            .all(|d| d.red <= size.red && d.blue <= size.blue && d.green <= size.green)
    }
    games
        .iter()
        .filter(|&g| is_possible(&game_size, g))
        .map(|g| g.id)
        .sum()
}

fn part_2(games: &Vec<Game>) -> u64 {
    fn min_possible(game: &Game) -> GameSize {
        game.draws.iter().fold(
            GameSize {
                red: 0,
                blue: 0,
                green: 0,
            },
            |acc, d| GameSize {
                red: acc.red.max(d.red),
                blue: acc.blue.max(d.blue),
                green: acc.green.max(d.green),
            },
        )
    }
    games
        .iter()
        .map(min_possible)
        .map(|s| s.red * s.blue * s.green)
        .sum()
}

fn read_games(input: &str) -> Result<Vec<Game>, String> {
    input
        .lines()
        .map(read_game)
        .collect::<Result<Vec<Game>, String>>()
}

fn read_game(line: &str) -> Result<Game, String> {
    let mut game_and_draws = line.split(": ");
    let id = game_and_draws
        .next()
        .map(|s| &s[5..])
        .ok_or("No id found".to_owned())
        .and_then(|id| id.parse::<u64>().map_err(|e| e.to_string()))?;
    let draws = game_and_draws
        .next()
        .ok_or("No draws found")?
        .split("; ")
        .map(read_draw)
        .collect::<Result<Vec<Draw>, String>>()?;
    Ok(Game { id, draws })
}

fn read_draw(draw_spec: &str) -> Result<Draw, String> {
    let colors = draw_spec
        .split(", ")
        .map(|c| c.split(' '))
        .map(|mut num_color| match (num_color.next(), num_color.next()) {
            (Some(num), Some(color)) => num
                .parse::<u64>()
                .map_err(|e| e.to_string())
                .map(|num| (color, num)),
            _ => Err("could not read pair".to_owned()),
        })
        .collect::<Result<HashMap<&str, u64>, String>>()?;
    Ok(Draw {
        red: *colors.get("red").unwrap_or(&0),
        blue: *colors.get("blue").unwrap_or(&0),
        green: *colors.get("green").unwrap_or(&0),
    })
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u64,
    draws: Vec<Draw>,
}
#[derive(PartialEq, Debug)]
struct Draw {
    red: u64,
    blue: u64,
    green: u64,
}
struct GameSize {
    red: u64,
    blue: u64,
    green: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_game_test() {
        let expected = Game {
            id: 3,
            draws: vec![
                Draw {
                    green: 8,
                    blue: 6,
                    red: 20,
                },
                Draw {
                    blue: 5,
                    red: 4,
                    green: 13,
                },
                Draw {
                    green: 5,
                    red: 1,
                    blue: 0,
                },
            ],
        };
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(expected, read_game(input).unwrap())
    }

    #[test]
    fn part_1_test() -> Result<(), String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = read_games(input)?;
        let expected = 8;
        match part_1(&games) {
            n if n == expected => Ok(()),
            e => Err(format!("Expected {expected}, got {e}")),
        }
    }

    #[test]
    fn part_2_test() -> Result<(), String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = read_games(input)?;
        let expected = 2286;
        match part_2(&games) {
            n if n == expected => Ok(()),
            e => Err(format!("Expected {expected}, got {e}")),
        }
    }
}
