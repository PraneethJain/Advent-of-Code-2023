use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> i32 {
    let mut result = 0;
    for line in lines.lines() {
        let first_digit = line.chars().find(|c| c.is_numeric());
        let last_digit = line.chars().rev().find(|c| c.is_numeric());

        let num: Option<i32> = match (first_digit, last_digit) {
            (Some(first_digit), Some(last_digit)) => {
                format!("{}{}", first_digit, last_digit).parse().ok()
            }
            _ => None,
        };
        result += match num {
            Some(val) => val,
            None => panic!("error finding digits"),
        };
    }

    result
}

fn part_two(lines: &str) -> i32 {
    let possible: HashMap<String, i32> = vec![
        ("1".to_string(), 1),
        ("one".to_string(), 1),
        ("2".to_string(), 2),
        ("two".to_string(), 2),
        ("3".to_string(), 3),
        ("three".to_string(), 3),
        ("4".to_string(), 4),
        ("four".to_string(), 4),
        ("5".to_string(), 5),
        ("five".to_string(), 5),
        ("6".to_string(), 6),
        ("six".to_string(), 6),
        ("7".to_string(), 7),
        ("seven".to_string(), 7),
        ("8".to_string(), 8),
        ("eight".to_string(), 8),
        ("9".to_string(), 9),
        ("nine".to_string(), 9),
    ]
    .into_iter()
    .collect();

    let mut result = 0;
    for line in lines.lines() {
        let mut first = 0;
        let mut found_first = false;
        for i in 0..line.len() {
            if found_first {
                break;
            }

            for (key, val) in possible.iter() {
                if i + key.len() <= line.len() && line[i..i + key.len()] == *key {
                    first = *val;
                    found_first = true;
                    break;
                }
            }
        }

        let mut last = first;
        let mut found_last = false;
        for i in (0..line.len()).rev() {
            if found_last {
                break;
            }

            for (key, val) in possible.iter() {
                if i + key.len() <= line.len() && line[i..i + key.len()] == *key {
                    last = *val;
                    found_last = true;
                    break;
                }
            }
        }

        result += match format!("{}{}", first, last).parse::<i32>() {
            Ok(val) => val,
            Err(_) => panic!("couldn't find any digits in line {}", line),
        }
    }
    result
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
