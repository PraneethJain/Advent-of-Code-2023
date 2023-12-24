mod fraction;
mod line;

use fraction::Fraction;
use itertools::Itertools;
use line::Line;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> usize {
    let valid = |&f: &Fraction, a: i128, b: i128| -> bool {
        f >= Fraction::new(200000000000000, 1)
            && f <= Fraction::new(400000000000000, 1)
            && (f / b - Fraction::new(a, b)) > Fraction::new(0, 1)
    };

    lines
        .lines()
        .map(|line| match line.split_once('@') {
            Some((l, r)) => {
                let position: Vec<_> = l
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect();
                let velocity: Vec<_> = r
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect();
                (
                    Line::new(
                        Fraction::new(velocity[1], velocity[0]),
                        Fraction::new(position[1], 1)
                            - Fraction::new(velocity[1], velocity[0]) * position[0],
                    ),
                    position[0],
                    position[1],
                    velocity[0],
                    velocity[1],
                )
            }
            None => panic!("no @ in line {}", line),
        })
        .combinations(2)
        .filter(|pair| {
            let (x, y) = pair[0].0.solve(&pair[1].0);
            [0, 1].iter().all(|&idx| {
                valid(&x, pair[idx].1, pair[idx].3) && valid(&y, pair[idx].2, pair[idx].4)
            })
        })
        .count()
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut lines = String::new();

    if let Err(why) = file.read_to_string(&mut lines) {
        panic!("couldn't read {}: {}", display, why)
    }

    println!("{}", part_one(&lines));
}
