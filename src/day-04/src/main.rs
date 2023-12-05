use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> i32 {
    let mut result = 0;

    for line in lines.lines() {
        let data = match line.trim().split_once(':') {
            Some((_, data)) => data.trim(),
            None => panic!("invalid input, found no :"),
        };

        match data.split_once('|') {
            Some((left, right)) => {
                let left_vec: Vec<&str> = left.trim().split_whitespace().collect();
                let left_set: BTreeSet<String> = left_vec.into_iter().map(String::from).collect();

                let right_vec: Vec<&str> = right.trim().split_whitespace().collect();
                let right_set: BTreeSet<String> = right_vec.into_iter().map(String::from).collect();

                let intersection: BTreeSet<_> = left_set.intersection(&right_set).collect();
                result += match intersection.len() {
                    0 => 0,
                    count => 1 << (count - 1),
                }
            }
            None => panic!("invalid input, found no |"),
        };
    }

    result
}

fn part_two(lines: &str) -> i32 {
    let mut times = vec![1; lines.lines().count()];
    for (i, line) in lines.lines().enumerate() {
        let data = match line.trim().split_once(':') {
            Some((_, data)) => data.trim(),
            None => panic!("invalid input, found no :"),
        };

        match data.split_once('|') {
            Some((left, right)) => {
                let left_vec: Vec<&str> = left.trim().split_whitespace().collect();
                let left_set: BTreeSet<String> = left_vec.into_iter().map(String::from).collect();

                let right_vec: Vec<&str> = right.trim().split_whitespace().collect();
                let right_set: BTreeSet<String> = right_vec.into_iter().map(String::from).collect();

                let intersection: BTreeSet<_> = left_set.intersection(&right_set).collect();
                match intersection.len() {
                    0 => (),
                    count => {
                        for j in (i + 1)..(i + 1 + count) {
                            if j >= times.len() {
                                break;
                            }
                            times[j] += times[i];
                        }
                    }
                }
            }
            None => panic!("invalid input, found no |"),
        };
    }

    times.iter().sum()
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display, why),
    };

    let mut lines = String::new();
    match file.read_to_string(&mut lines) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        _ => {}
    };

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
