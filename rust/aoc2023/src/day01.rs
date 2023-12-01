pub fn run(input: &str) -> Result<(), String> {
    let part1: u32 = calibrate(input, to_digits_part1)?;
    println!("Part 1: {part1}");
    let part2: u32 = calibrate(input, to_digits_part2)?;
    println!("Part 2: {part2}");
    Ok(())
}

fn calibrate(input: &str, read: fn(&str) -> Vec<u32>) -> Result<u32, &str> {
    input.lines().map(read).map(calibration_value).sum()
}

fn calibration_value(digits: Vec<u32>) -> Result<u32, &'static str> {
    match (digits.first(), digits.last()) {
        (Some(f), Some(l)) => Ok(f * 10 + l),
        _ => Err("not enough digits"),
    }
}

fn to_digits_part1(line: &str) -> Vec<u32> {
    line.chars().filter_map(|x| x.to_digit(10)).collect()
}

fn to_digits_part2(line: &str) -> Vec<u32> {
    fn extract_digits(chars: &[char], acc: Vec<u32>) -> Vec<u32> {
        match chars {
            [] => acc,
            [h, rest @ ..] => {
                let acc_new = match h.to_digit(10).or_else(|| find_digit_word(chars)) {
                    Some(x) => [acc, vec![x]].concat(),
                    _ => acc,
                };
                extract_digits(rest, acc_new)
            }
        }
    }
    let chars = line.chars().collect::<Vec<_>>();
    extract_digits(&chars[..], vec![])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_digits_part1_examples() {
        assert_eq!(vec![1, 2], to_digits_part1("1abc2"));
        assert_eq!(vec![3, 8], to_digits_part1("pqr3stu8vwx"));
        assert_eq!(vec![1, 2, 3, 4, 5], to_digits_part1("a1b2c3d4e5f"));
        assert_eq!(vec![7], to_digits_part1("treb7uchet"));
    }

    #[test]
    fn to_digits_part2_examples() {
        assert_eq!(vec![2, 1, 9], to_digits_part2("two1nine"));
        assert_eq!(vec![8, 2, 3], to_digits_part2("eightwothree"));
        assert_eq!(vec![1, 2, 3], to_digits_part2("abcone2threexyz"));
        assert_eq!(vec![2, 1, 3, 4], to_digits_part2("xtwone3four"));
        assert_eq!(vec![4, 9, 8, 7, 2], to_digits_part2("4nineeightseven2"));
        assert_eq!(vec![1, 8, 2, 3, 4], to_digits_part2("zoneight234"));
        assert_eq!(vec![7, 6], to_digits_part2("7pqrstsixteen"));
    }

    #[test]
    fn calibration_value_test() -> Result<(), String> {
        assert_eq!(29, calibration_value(vec![2, 1, 9])?);
        assert_eq!(77, calibration_value(vec![7])?);
        assert_eq!(42, calibration_value(vec![4, 9, 8, 7, 2])?);
        assert!(calibration_value(vec![]).is_err());
        Ok(())
    }
}
