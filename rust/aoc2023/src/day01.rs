pub fn run(input: &str) -> Result<(), String> {
    let part1: u32 = calibrate(input, to_digits_part1);
    println!("Part 1: {part1}");
    let part2: u32 = calibrate(input, to_digits_part2);
    println!("Part 2: {part2}");
    Ok(())
}

fn calibrate(input: &str, read: fn(&str) -> Vec<u32>) -> u32 {
    input.lines().map(read).map(calibration_value).sum()
}

fn calibration_value(digits: Vec<u32>) -> u32 {
    match (digits.first(), digits.last()) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => panic!("not enough digits"),
    }
}

fn to_digits_part1(line: &str) -> Vec<u32> {
    line.chars().filter_map(|x| x.to_digit(10)).collect()
}

fn to_digits_part2(line: &str) -> Vec<u32> {
    let chars = line.chars().collect::<Vec<_>>();
    extract_digits(&chars[..], vec![])
}

fn extract_digits(chars: &[char], result: Vec<u32>) -> Vec<u32> {
    match chars {
        [] => result,
        [h, rest @ ..] => {
            let new_result = match h.to_digit(10).or_else(|| find_digit_word(chars)) {
                Some(x) => [result, vec![x]].concat(),
                _ => result,
            };
            extract_digits(rest, new_result)
        }
    }
}

const WORD_TO_DIGIT: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
fn find_digit_word(chars: &[char]) -> Option<u32> {
    WORD_TO_DIGIT
        .iter()
        .find(|(word, _)| String::from_iter(chars.iter().take(word.len())).eq(word))
        .map(|x| x.1)
}
