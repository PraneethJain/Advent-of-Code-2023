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
            println!("{}", y);
            y
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
