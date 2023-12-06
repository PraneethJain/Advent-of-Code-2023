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
        .map(|&s| match s.parse::<i64>() {
            Ok(x) => x,
            Err(why) => panic!("couldn't parse {} as i64: {}", s, why),
        })
        .collect();

    iter_lines.next();
    iter_lines.next();
    let mut new: BTreeSet<i64> = BTreeSet::new();
    while let Some(line) = iter_lines.next() {
        if line.trim().is_empty() {
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

fn part_two(lines: &str) -> i64 {
    let mut iter_lines = lines.lines();
    let seeds_strs: Vec<&str> = match iter_lines.next() {
        Some(line) => match line.split_once(":") {
            Some((_, right)) => right.trim().split_whitespace().into_iter().collect(),
            None => panic!("couldn't find : in first line"),
        },
        None => panic!("first line does not exist"),
    };

    iter_lines.next();
    iter_lines.next();
    let mut mappings: Vec<Vec<(i64, i64, i64)>> = vec![vec![]];
    while let Some(line) = iter_lines.next() {
        if line.trim().is_empty() {
            mappings.push(vec![]);
            iter_lines.nth(0);
            continue;
        }

        let vector: Vec<i64> = line
            .split_whitespace()
            .map(|s| match s.parse::<i64>() {
                Ok(x) => x,
                Err(why) => panic!("couldn't parse {} as i64: {}", s, why),
            })
            .collect();

        match mappings.last_mut() {
            Some(v) => v.push((vector[0], vector[1], vector[1] + vector[2])),
            None => panic!("couldn't parse line: {}", line),
        }
    }

    let mut old: BTreeSet<(i64, i64)> = seeds_strs
        .chunks_exact(2)
        .map(|chunk| {
            (
                chunk[0].parse::<i64>().unwrap(),
                chunk[0].parse::<i64>().unwrap() + chunk[1].parse::<i64>().unwrap(),
            )
        })
        .collect();
    for mapping in mappings {
        let mut new: BTreeSet<(i64, i64)> = BTreeSet::new();
        while !old.is_empty() {
            let (start, end) = match old.pop_first() {
                Some((start, end)) => (start, end),
                None => panic!("popping from empty set"),
            };

            let mut found = false;
            for &(to_start, from_start, from_end) in &mapping {
                let final_start = start.max(from_start);
                let final_end = end.min(from_end);

                if final_end > final_start {
                    new.insert((
                        final_start - from_start + to_start,
                        final_end - from_start + to_start,
                    ));
                    if final_start > start {
                        old.insert((start, final_start));
                    }

                    if final_end < end {
                        old.insert((final_end, end));
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                new.insert((start, end));
            }
        }
        old.clear();
        old.append(&mut new.clone());
    }
    match old.iter().min() {
        Some(&(x, _)) => x,
        None => panic!("result is empty"),
    }
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
    println!("{}", part_two(&lines));
}
