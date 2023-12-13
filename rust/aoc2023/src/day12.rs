use itertools::Itertools;

use crate::util::read_all_sep_by;

pub fn run(input: &str) -> Result<(), String> {
    let springs = read_input(input)?;
    let part1 = count_possibilities(&springs);
    println!("Part 1: {}", part1);
    let springs_unfolded = unfold_read(input)?;
    //let part2 = count_possibilities(&springs_unfolded);
    println!("Part 2: {}", "todo");
    Ok(())
}

fn count_possibilities(springs: &Vec<(Vec<char>, Vec<i64>)>) -> i64 {
    springs
        .iter()
        .map(|(row, spec)| possible_configs(row, spec))
        .sum()
}

#[derive(Debug)]
struct RowState {
    remaining: i64,
    springs: Vec<usize>,
}
impl RowState {
    fn choose(&self, position: usize) -> RowState {
        let mut springs = self.springs.clone();
        springs.push(position);
        RowState {
            remaining: self.remaining - 1,
            springs,
        }
    }
}
#[derive(Debug)]
struct RowInfo<'a> {
    row: &'a Vec<char>,
    spec: &'a Vec<i64>,
    open_spots: Vec<usize>,
}
fn possible_configs(row: &Vec<char>, spec: &Vec<i64>) -> i64 {
    let required_springs: i64 = spec.iter().sum();
    let open_spots = row
        .iter()
        .enumerate()
        .filter(|&c| *c.1 == '?')
        .map(|c| c.0)
        .collect_vec();
    let current_springs = row.iter().filter(|&c| *c == '#').count() as i64;
    let remaining_springs = required_springs - current_springs;
    let row_info = RowInfo {
        row,
        spec,
        open_spots,
    };
    let row_state = RowState {
        remaining: remaining_springs,
        springs: vec![],
    };

    fn possibilities(state: RowState, row: &RowInfo, next: usize) -> i64 {
        //println!("possibilities {:?} {:?} {next}", state, row);
        if 0 == state.remaining {
            return check_state(&state, row);
        }
        if row.open_spots.len() <= next {
            return 0;
        }
        let position = row.open_spots[next];
        return possibilities(state.choose(position), row, next + 1)
            + possibilities(state, row, next + 1);
    }
    fn check_state(state: &RowState, row: &RowInfo) -> i64 {
        let is_ok = (0..row.row.len())
            .map(|i| state.springs.contains(&i) || row.row[i] == '#')
            .group_by(|&v| v)
            .into_iter()
            .filter(|(is_spring, _)| *is_spring)
            .map(|(_, run)| run.count() as i64)
            .collect_vec()
            .eq(row.spec);
        if is_ok {
            1
        } else {
            0
        }
    }
    possibilities(row_state, &row_info, 0)
}

fn read_input(input: &str) -> Result<Vec<(Vec<char>, Vec<i64>)>, String> {
    input.lines().map(read_spring_row).try_collect()
}

fn read_spring_row(line: &str) -> Result<(Vec<char>, Vec<i64>), String> {
    let (springs, check) = line
        .split_once(' ')
        .ok_or_else(|| format!("Could not read line '{line}'"))?;
    read_all_sep_by(check, ',').map(|counts| (springs.chars().collect_vec(), counts))
}

fn unfold_read(input: &str) -> Result<Vec<(Vec<char>, Vec<i64>)>, String> {
    fn unfold(value: &str, sep: char) -> String {
        let mut unfolded = value.to_string();
        unfolded.push(sep);
        unfolded = unfolded.repeat(5);
        unfolded.pop();
        unfolded
    }
    fn unfold_read_line(line: &str) -> Result<(Vec<char>, Vec<i64>), String> {
        let (springs, check) = line
            .split_once(' ')
            .ok_or_else(|| format!("Could not read line '{line}'"))?;
        read_all_sep_by(unfold(check, ',').as_str(), ',')
            .map(|counts| (unfold(springs, '?').chars().collect_vec(), counts))
    }
    input.lines().map(unfold_read_line).try_collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_test() {
        assert_eq!(21, count_possibilities(&read_input(SAMPLE_INPUT).unwrap()));
    }
}
