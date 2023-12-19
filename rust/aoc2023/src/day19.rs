use std::{collections::HashMap, iter::successors};

use itertools::Itertools;
use regex::Regex;

use crate::util::read;

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: String,
}

#[derive(Debug)]
struct Rule {
    prop: String,
    op: String,
    value: i64,
    destination: String,
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

pub fn run(input: &str) -> Result<(), String> {
    let (workflows, parts) = read_input(input)?;
    let part1 = part1(&parts, &workflows);
    println!("Part 1: {part1}");
    Ok(())
}

fn part1(parts: &Vec<Part>, workflows: &Vec<Workflow>) -> i64 {
    let workflow_map = workflows
        .iter()
        .map(|w| (w.name.to_owned(), w))
        .collect::<HashMap<_, _>>();
    parts
        .iter()
        .filter(|p| is_accepted(p, &workflow_map))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

fn is_accepted(part: &Part, workflow_map: &HashMap<String, &Workflow>) -> bool {
    successors(Some("in".to_owned()), |prv| {
        workflow_map.get(prv).and_then(|workflow| {
            workflow
                .rules
                .iter()
                .find_map(|r| check_condition(r, part))
                .or_else(|| Some(workflow.default.to_owned()))
        })
    })
    .last()
    .unwrap()
    .eq("A")
}

fn check_condition(
    Rule {
        prop,
        op,
        value,
        destination,
    }: &Rule,
    part: &Part,
) -> Option<String> {
    let lhs = match prop.as_str() {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => panic!("Not a prop: {prop}"),
    };
    Some(destination)
        .filter(|_| match op.as_str() {
            "<" => lhs < *value,
            ">" => lhs > *value,
            _ => panic!("Not an op: {op}"),
        })
        .cloned()
}

fn read_input(input: &str) -> Result<(Vec<Workflow>, Vec<Part>), String> {
    match input.split_once("\n\n") {
        Some((w, p)) => read_workflows(w).and_then(|ws| read_parts(p).map(|ps| (ws, ps))),
        None => Err("Could not read input".to_string()),
    }
}

fn read_workflows(spec: &str) -> Result<Vec<Workflow>, String> {
    let workflow_re = Regex::new(r"([a-z]+)\{((?:[^,]+,)*)([a-zAR]+)\}").unwrap();
    let rules_re = Regex::new(r"([xmas])([<>])(\d+):([a-zAR]+)").unwrap();
    spec.lines()
        .map(|l| {
            workflow_re
                .captures(l)
                .map(|c| c.extract())
                .ok_or_else(|| format!("Could not read line {l}"))
                .and_then(|(_, [name, rules, default])| {
                    read_rules(&rules_re, &rules.to_owned()).map(|rs| Workflow {
                        name: name.to_owned(),
                        rules: rs,
                        default: default.to_owned(),
                    })
                })
        })
        .try_collect()
}

fn read_rules(re: &Regex, spec: &str) -> Result<Vec<Rule>, String> {
    let result = re
        .captures_iter(spec)
        .map(|c| c.extract())
        .map(|(_, [prop, op, val, dest])| {
            read::<i64>(val).map(|v| Rule {
                prop: prop.to_owned(),
                op: op.to_owned(),
                value: v,
                destination: dest.to_owned(),
            })
        })
        .try_collect();
    result
}

fn read_parts(spec: &str) -> Result<Vec<Part>, String> {
    let parts_re = Regex::new(r"(?m)^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}$").unwrap();
    parts_re
        .captures_iter(spec)
        .map(|c| c.extract())
        .map(|(_, [x, m, a, s])| {
            Ok(Part {
                x: read(x)?,
                m: read(m)?,
                a: read(a)?,
                s: read(s)?,
            })
        })
        .try_collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_test() {
        let (w, p) = read_input(SAMPLE_INPUT).unwrap();
        assert_eq!(19114, part1(&p, &w))
    }
}
