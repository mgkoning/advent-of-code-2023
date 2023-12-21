use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

struct ModuleDef<'a> {
    kind: char,
    name: String,
    outputs: Vec<&'a str>,
}
#[derive(Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
    SandMachine(bool),
}
impl Module {
    fn receive(&self, from: String, pulse: bool) -> (Module, Option<bool>) {
        match self {
            Self::Broadcaster => (Self::Broadcaster, Some(pulse)),
            Self::FlipFlop(state) if !pulse => (Self::FlipFlop(!state), Some(!state)),
            f @ Self::FlipFlop(..) => (f.clone(), None),
            Self::Conjunction(state) => {
                let mut new_state = state.clone();
                new_state.entry(from).and_modify(|p| *p = pulse);
                let pulse = !new_state.values().all(|&on| on);
                (Self::Conjunction(new_state), Some(pulse))
            }
            Self::SandMachine(..) => (Self::SandMachine(pulse), None),
        }
    }
}
struct Message {
    sender: String,
    recipient: String,
    pulse: bool,
}
struct PulseCounter {
    low: usize,
    high: usize,
}
impl PulseCounter {
    fn count(&mut self, pulse: bool) {
        if pulse {
            self.high += 1
        } else {
            self.low += 1
        }
    }
}

pub fn run(input: &str) -> Result<(), String> {
    let modules = read_modules(input);
    let part1 = part1(&modules);
    println!("Part 1: {}", part1);
    let part2 = part2(&modules);
    println!("Part 2: {}", part2);
    Ok(())
}

fn part1(modules: &[ModuleDef<'_>]) -> i64 {
    let (edges, mut state) = initial_state(modules);
    let mut counter = PulseCounter { low: 0, high: 0 };
    for _ in 0..1000 {
        push_button(&mut counter, &mut state, &edges);
    }
    (counter.low * counter.high) as i64
}

fn part2(modules: &[ModuleDef<'_>]) -> i64 {
    let (edges, mut state) = initial_state(modules);
    let mut counter = PulseCounter { low: 0, high: 0 };
    state.insert("rx".to_string(), Module::SandMachine(true));
    for i in 1.. {
        push_button(&mut counter, &mut state, &edges);
        let sandmachine = state.get("rx").unwrap();
        match sandmachine {
            Module::SandMachine(pulse) if !pulse => return i,
            _ => {}
        }
    }
    -1
}

fn push_button(
    counter: &mut PulseCounter,
    state: &mut HashMap<String, Module>,
    edges: &Vec<(String, String)>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(Message {
        sender: "button".to_owned(),
        recipient: "broadcaster".to_owned(),
        pulse: false,
    });
    while let Some(m) = queue.pop_front() {
        let Message {
            sender,
            recipient,
            pulse,
        } = m;
        counter.count(pulse);
        state.entry(recipient.clone()).and_modify(|m| {
            let (new_module, output) = m.receive(sender, pulse);
            *m = new_module;
            if let Some(signal) = output {
                edges
                    .iter()
                    .filter(|(from, _)| recipient.eq(from))
                    .map(|(_, to)| to)
                    .for_each(|r| {
                        queue.push_back(Message {
                            sender: recipient.to_string(),
                            recipient: r.to_owned(),
                            pulse: signal,
                        })
                    })
            }
        });
    }
}

fn initial_state(modules: &[ModuleDef<'_>]) -> (Vec<(String, String)>, HashMap<String, Module>) {
    let edges = modules
        .iter()
        .flat_map(|m| m.outputs.iter().map(|o| (m.name.to_owned(), o.to_string())))
        .collect_vec();
    let state = modules
        .iter()
        .map(|m| {
            let module = match m.kind {
                'b' => Module::Broadcaster,
                '%' => Module::FlipFlop(false),
                '&' => Module::Conjunction(
                    edges
                        .iter()
                        .filter(|(_, to)| m.name.eq(to))
                        .map(|(from, _)| (from.to_owned(), false))
                        .collect(),
                ),
                _ => panic!("unknown kind {}", m.kind),
            };
            (m.name.to_owned(), module)
        })
        .collect::<HashMap<_, _>>();
    (edges, state)
}

fn read_modules(input: &str) -> Vec<ModuleDef> {
    let line_re = Regex::new(r"(?m)^([a-z%&]+) -> ([a-z, ]+)$").unwrap();
    line_re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [decl, rhs])| {
            let kind = decl.chars().nth(0).unwrap();
            let name = match kind {
                'b' => decl.to_owned(),
                _ => decl.chars().skip(1).collect::<String>(),
            };
            let outputs = rhs.split(", ").collect_vec();
            ModuleDef {
                kind,
                name,
                outputs,
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const SAMPLE_INPUT_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_test() {
        assert_eq!(32000000, part1(&read_modules(SAMPLE_INPUT_1)));
        assert_eq!(11687500, part1(&read_modules(SAMPLE_INPUT_2)));
    }
}
