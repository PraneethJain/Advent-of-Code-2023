use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> i64 {
    let mut iter_lines = lines.lines();
    let seeds_strs: Vec<&str> = match iter_lines.next() {
        Some(line) => match line.split_once(":") {
            Some((_, right)) => right.trim().split_whitespace().into_iter().collect(),
            None => panic!("couldn't find : in first line"),
        },
        None => panic!("first line does not exist"),
    };
    let mut old: BTreeSet<i64> = seeds_strs
        .iter()
        .map(|&s| s.parse::<i64>().unwrap())
        .collect();

    iter_lines.next();
    iter_lines.next();
    let mut new: BTreeSet<i64> = BTreeSet::new();
    while let Some(line) = iter_lines.next() {
        if line.trim().is_empty() {
            new.append(&mut old.clone());
            old.clear();
            old.append(&mut new.clone());
            new.clear();
            iter_lines.nth(0);
            continue;
        }

        let vector: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let (to_start, from_start, length) = (vector[0], vector[1], vector[2]);

        let mut to_remove: BTreeSet<i64> = BTreeSet::new();
        for &val in &old {
            if val >= from_start && val < from_start + length {
                new.insert(val - from_start + to_start);
                to_remove.insert(val);
            }
        }
        old.retain(|x| !to_remove.contains(x));
    }
    new.append(&mut old.clone());
    *new.iter().min().unwrap()
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
