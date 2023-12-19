use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Condition {
    letter: char,
    comparison: char,
    value: i32,
    result: String,
}

#[derive(Debug)]
struct Workflow {
    conditions: Vec<Condition>,
    fallback: String,
}

fn part_one(lines: &str) -> i32 {
    let (workflow_lines, rating_lines) = lines
        .split_once("\n\n")
        .expect("no new empty line in input");

    let workflows: BTreeMap<&str, Workflow> = workflow_lines
        .lines()
        .map(|line| {
            let (name, condition_string) = line[..(line.len() - 1)]
                .split_once('{')
                .expect("no { in line");
            let mut condition_strings: Vec<_> = condition_string.split(',').collect();
            (
                name,
                Workflow {
                    fallback: condition_strings
                        .pop()
                        .expect("no fallback found")
                        .to_string(),
                    conditions: condition_strings
                        .iter()
                        .map(|&cond| {
                            let (l, r) = cond[2..].split_once(':').expect("no : in condition");
                            Condition {
                                letter: cond.chars().nth(0).expect("0th character"),
                                comparison: cond.chars().nth(1).expect("1th character"),
                                value: l.parse().unwrap(),
                                result: r.to_string(),
                            }
                        })
                        .collect(),
                },
            )
        })
        .collect();

    rating_lines
        .lines()
        .map(|line| {
            let rating: BTreeMap<char, i32> = line[1..(line.len() - 1)]
                .split(',')
                .map(|rat| {
                    let (l, r) = rat.split_once('=').expect("no = in rating");
                    (l.chars().nth(0).expect("0th character"), r.parse().unwrap())
                })
                .collect();
            let mut cur = "in";
            loop {
                let workflow = workflows.get(&cur).unwrap();
                let mut followed = false;
                for condition in &workflow.conditions {
                    if match condition.comparison {
                        '>' => rating[&condition.letter] > condition.value,
                        '<' => rating[&condition.letter] < condition.value,
                        c => panic!("found character {} in comparison", c),
                    } {
                        followed = true;
                        cur = condition.result.as_str();
                        break;
                    }
                }

                if !followed {
                    cur = workflow.fallback.as_str();
                }

                if "AR".contains(cur) {
                    break;
                }
            }
            match cur {
                "A" => rating.values().sum(),
                "R" => 0,
                s => panic!("found {}", s),
            }
        })
        .sum()
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut lines = String::new();
    match file.read_to_string(&mut lines) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        _ => {}
    }

    println!("{}", part_one(&lines));
}
