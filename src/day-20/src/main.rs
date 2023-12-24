use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(bool, Vec<String>),
    Conjunction(BTreeMap<String, bool>, Vec<String>),
}

fn button_press(
    modules: &mut BTreeMap<String, Module>,
    q: &VecDeque<(String, String, bool)>,
) -> (i32, i32) {
    let mut low = 1;
    let mut high = 0;
    let mut q = q.clone();
    while !q.is_empty() {
        let (source, cur, strength) = q.pop_front().expect("queue is empty");
        match strength {
            true => high += 1,
            false => low += 1,
        }
        match modules.get_mut(&cur) {
            Some(Module::FlipFlop(state, destinations)) => {
                if !strength {
                    *state = !*state;
                    for destination in destinations {
                        q.push_back((cur.to_string(), destination.to_string(), *state));
                    }
                }
            }
            Some(Module::Conjunction(state, destinations)) => {
                *state.get_mut(&source).expect("source not initialized") = strength;
                let pulse = !state.values().all(|&x| x);
                for destination in destinations {
                    q.push_back((cur.to_string(), destination.to_string(), pulse));
                }
            }
            None => (),
        }
    }
    (low, high)
}

fn part_one(lines: &str) -> i32 {
    let mut modules: BTreeMap<String, Module> = BTreeMap::new();
    let mut q: VecDeque<(String, String, bool)> = VecDeque::new();
    for line in lines.lines() {
        let (source, destinations) = match line.split_once("->") {
            Some((from, tos)) => (from.trim(), tos.trim()),
            None => panic!("no -> in line {}", line),
        };
        let vec: Vec<_> = destinations
            .trim()
            .split(',')
            .map(|x| x.trim().to_string())
            .collect();

        if let Some(stripped) = source.strip_prefix('%') {
            modules.insert(stripped.to_string(), Module::FlipFlop(false, vec));
        } else if let Some(stripped) = source.strip_prefix('&') {
            modules.insert(
                stripped.to_string(),
                Module::Conjunction(BTreeMap::new(), vec),
            );
        } else if source == "broadcaster" {
            for destination in vec {
                q.push_back((source.to_string(), destination.to_string(), false));
            }
        } else {
            panic!("found {} in from", source);
        }
    }

    for (x, y) in modules.clone() {
        let destinations = match y {
            Module::FlipFlop(_, destinations) => destinations,
            Module::Conjunction(_, destinations) => destinations,
        };

        for destination in destinations {
            if let Some(module) = modules.get_mut(&destination) {
                match module {
                    Module::FlipFlop(_, _) => (),
                    Module::Conjunction(state, _) => {
                        state.insert(x.to_string(), false);
                    }
                }
            }
        }
    }
    let mut total_low = 0;
    let mut total_high = 0;
    for _ in 0..1000 {
        let (low, high) = button_press(&mut modules, &q);
        total_low += low;
        total_high += high;
    }
    total_low * total_high
}

fn button_press_till_rx(
    modules: &mut BTreeMap<String, Module>,
    q: &VecDeque<(String, String, bool)>,
    cycles: &mut BTreeMap<String, i64>,
    count: i64,
) -> bool {
    let mut q = q.clone();
    while !q.is_empty() {
        let (source, cur, strength) = q.pop_front().expect("queue is empty");

        // hardcoded from input
        if cur == "lx" && strength && !cycles.contains_key(&source) {
            cycles.insert(source.clone(), count);
        }
        // hardcoded from input
        if cycles.len() == 4 {
            return true;
        }

        match modules.get_mut(&cur) {
            Some(Module::FlipFlop(state, destinations)) => {
                if !strength {
                    *state = !*state;
                    for destination in destinations {
                        q.push_back((cur.to_string(), destination.to_string(), *state));
                    }
                }
            }
            Some(Module::Conjunction(state, destinations)) => {
                *state.get_mut(&source).expect("source not initialized") = strength;
                let pulse = !state.values().all(|&x| x);
                for destination in destinations {
                    q.push_back((cur.to_string(), destination.to_string(), pulse));
                }
            }
            None => (),
        }
    }
    false
}

fn part_two(lines: &str) -> i64 {
    let mut modules: BTreeMap<String, Module> = BTreeMap::new();
    let mut q: VecDeque<(String, String, bool)> = VecDeque::new();
    for line in lines.lines() {
        let (source, destinations) = match line.split_once("->") {
            Some((from, tos)) => (from.trim(), tos.trim()),
            None => panic!("no -> in line {}", line),
        };
        let vec: Vec<_> = destinations
            .trim()
            .split(',')
            .map(|x| x.trim().to_string())
            .collect();

        if let Some(stripped) = source.strip_prefix('%') {
            modules.insert(stripped.to_string(), Module::FlipFlop(false, vec));
        } else if let Some(stripped) = source.strip_prefix('&') {
            modules.insert(
                stripped.to_string(),
                Module::Conjunction(BTreeMap::new(), vec),
            );
        } else if source == "broadcaster" {
            for destination in vec {
                q.push_back((source.to_string(), destination.to_string(), false));
            }
        } else {
            panic!("found {} in from", source);
        }
    }

    for (x, y) in modules.clone() {
        let destinations = match y {
            Module::FlipFlop(_, destinations) => destinations,
            Module::Conjunction(_, destinations) => destinations,
        };

        for destination in destinations {
            if let Some(module) = modules.get_mut(&destination) {
                match module {
                    Module::FlipFlop(_, _) => (),
                    Module::Conjunction(state, _) => {
                        state.insert(x.to_string(), false);
                    }
                }
            }
        }
    }

    let mut cycles: BTreeMap<String, i64> = BTreeMap::new();
    let mut count = 1;
    while !button_press_till_rx(&mut modules, &q, &mut cycles, count) {
        count += 1;
    }
    cycles.values().product()
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("couldn't read file");

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
