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
