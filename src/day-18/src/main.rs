use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn shoelace(points: &Vec<(i32, i32)>) -> i32 {
    points
        .windows(2)
        .fold(0, |acc, ps| acc + ps[0].0 * ps[1].1 - ps[0].1 * ps[1].0)
        .abs()
        / 2
}

fn part_one(lines: &str) -> i32 {
    let mut cur = (0, 0);
    let mut points = vec![(0, 0)];
    let mut p = 0;
    for line in lines.lines() {
        let parts: Vec<_> = line.split_whitespace().map(|x| x.trim()).collect();
        let num: i32 = parts[1].parse().expect("couldn't parse to i32");
        p += num;
        match parts[0] {
            "U" => cur.1 += num,
            "R" => cur.0 += num,
            "D" => cur.1 -= num,
            "L" => cur.0 -= num,
            other => panic!("found {}", other),
        };
        points.push(cur.clone());
    }
    // Pick's Theorem
    // area = i + p/2 - 1, where i is number of points inside, p is number of points on
    let area = shoelace(&points);
    let i = area - p / 2 + 1;
    i + p
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
