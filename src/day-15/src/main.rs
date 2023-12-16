use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn hash_str(string: &str) -> usize {
    let mut res = 0;
    for c in string.chars() {
        res += c as usize;
        res *= 17;
        res %= 256;
    }
    res
}

fn part_one(lines: &str) -> usize {
    lines
        .split(',')
        .map(|x| {
            let y = hash_str(x.trim());
            y
        })
        .sum()
}

fn part_two(lines: &str) -> usize {
    let mut boxes: Vec<Vec<&str>> = vec![vec![]; 256];
    let mut focal_lengths: HashMap<&str, usize> = HashMap::new();
    for line in lines.split(',') {
        if line.contains('=') {
            let (name, focal_length) = line.trim().split_once("=").expect("no = in input");
            focal_lengths.insert(
                name,
                focal_length
                    .parse::<usize>()
                    .expect("couldn't parse to usize"),
            );

            let box_number = hash_str(name);
            if !boxes[box_number].contains(&name) {
                boxes[box_number].push(name);
            }
        } else {
            let (name, _) = line.split_once('-').expect("no - in input");
            let box_number = hash_str(name);
            boxes[box_number].retain(|&x| x != name);
        }
    }

    boxes.iter().enumerate().fold(0, |acc, (box_num, boxx)| {
        acc + boxx.iter().enumerate().fold(0, |ac, (slot_num, name)| {
            ac + (box_num + 1)
                * (slot_num + 1)
                * focal_lengths.get(name).expect("focal length not found")
        })
    })
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
