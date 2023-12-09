use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> i32 {
    let mut iter_lines = lines.lines();
    let directions = match iter_lines.next() {
        Some(line) => line.trim(),
        None => panic!("input is empty"),
    };

    println!("{}", directions);
    iter_lines.next();
    let mut adjacency_list: HashMap<String, (String, String)> = HashMap::new();
    while let Some(line) = iter_lines.next() {
        match line.split_once("=") {
            Some((left, right)) => {
                match right
                    .replace("(", "")
                    .replace(")", "")
                    .replace(" ", "")
                    .split_once(",")
                {
                    Some((l, r)) => {
                        adjacency_list.insert(left.trim().to_owned(), (l.to_owned(), r.to_owned()));
                    }
                    None => panic!("couldn't find , in {}", right),
                }
            }
            None => panic!("couldn't find = in {}", line),
        }
    }
    let mut res = 0;
    let mut pos = "AAA".to_owned();
    'outer: loop {
        for dir in directions.chars() {
            let cur = &adjacency_list.get(&pos).unwrap();
            match dir {
                'L' => pos = cur.0.to_owned(),
                'R' => pos = cur.1.to_owned(),
                _ => panic!("invalid direction {}", dir),
            }
            res += 1;
            if pos == "ZZZ" {
                break 'outer;
            }
        }
    }
    res
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
