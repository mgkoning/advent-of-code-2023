use std::array;

use itertools::Itertools;

enum Instruction<'a> {
    Place(&'a str, u8),
    Remove(&'a str),
}
impl Instruction<'_> {
    fn label(&self) -> &str {
        match self {
            Self::Place(label, _) => label,
            Self::Remove(label) => label,
        }
    }
}

pub fn run(input: &str) -> Result<(), String> {
    let instructions = read_input(input);
    let part1: u64 = instructions.iter().map(|s| run_hash(s)).sum();
    println!("Part 1: {part1}");

    let part2 = part2(&instructions);
    println!("Part 2: {part2}");
    Ok(())
}

fn part2(instructions: &Vec<&str>) -> u64 {
    let mut boxes: [Vec<(&str, u8)>; 256] = array::from_fn(|_| vec![]);
    for instr in instructions {
        let instruction =
            read_instruction(instr).expect(format!("could not read {instr}").as_str());
        let label = instruction.label();
        let lenses = &mut boxes[run_hash(label) as usize];
        match (lenses.iter().position(|(l, _)| label.eq(*l)), instruction) {
            (None, Instruction::Remove(_)) => (),
            (None, Instruction::Place(label, lens)) => lenses.push((label, lens)),
            (Some(i), Instruction::Remove(_)) => {
                lenses.remove(i);
            }
            (Some(i), Instruction::Place(label, lens)) => lenses[i] = (label, lens),
        }
    }

    boxes
        .iter()
        .zip(1u64..)
        .flat_map(|(lenses, b)| {
            lenses
                .iter()
                .zip(1u64..)
                .map(move |((_, f), l)| b * l * (*f as u64))
        })
        .sum()
}

fn read_instruction(instruction: &str) -> Option<Instruction> {
    instruction
        .split_once('=')
        .map(|(label, focal_length)| {
            let lens = focal_length
                .chars()
                .nth(0)
                .and_then(|c| c.to_digit(10))
                .expect(format!("{focal_length} must be digit").as_str())
                as u8;
            Instruction::Place(label, lens)
        })
        .or_else(|| {
            instruction
                .split_once('-')
                .map(|(label, _)| Instruction::Remove(label))
        })
}

fn run_hash(value: &str) -> u64 {
    value
        .as_ascii()
        .expect(format!("expected '{value}' to be ascii-compatible").as_str())
        .iter()
        .fold(0u64, |acc, next| ((acc + next.to_u8() as u64) * 17 % 256))
}

fn read_input(input: &str) -> Vec<&str> {
    input.split(',').collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part2_test() {
        assert_eq!(145, part2(&read_input(SAMPLE_INPUT)));
    }
}
