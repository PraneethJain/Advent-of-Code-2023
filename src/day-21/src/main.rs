use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = vec![vec![' '; lines.first().map_or(0, |line| line.len())]; lines.len()];

    for (x, line) in lines.iter().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            grid[x][y] = ch;
        }
    }

    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn find_start(grid: &[Vec<char>]) -> Option<(i32, i32)> {
    for (x, row) in grid.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}

fn part_one(lines: &str) -> i32 {
    let grid = string_to_grid(lines);
    let start = find_start(&grid).expect("no start found");
    let mut seen: BTreeSet<_> = BTreeSet::from([start]);
    let mut q: VecDeque<_> = VecDeque::from([(start.0, start.1, 64)]);
    let mut res = 0;

    while !q.is_empty() {
        let (x, y, steps) = q.pop_front().expect("queue is empty");

        if steps % 2 == 0 {
            res += 1;
        }

        if steps > 0 {
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (xx, yy) = (x + dx, y + dy);
                if xx > 0
                    && xx < grid.len() as i32
                    && yy > 0
                    && yy < grid[0].len() as i32
                    && grid[xx as usize][yy as usize] == '.'
                    && !seen.contains(&(xx, yy))
                {
                    seen.insert((xx, yy));
                    q.push_back((xx, yy, steps - 1));
                }
            }
        }
    }

    res
}

fn count(grid: &[Vec<char>]) -> (i64, i64, i64) {
    let start = find_start(grid).expect("no start found");
    let mut q = BTreeSet::from([start]);

    let at =
        |x: i32, y: i32| -> char { grid[x.rem_euclid(131) as usize][y.rem_euclid(131) as usize] };

    let mut res = Vec::new();
    let mut i = 0;
    loop {
        if i % 131 == 65 {
            res.push(q.len() as i64);
            if res.len() == 3 {
                break (res[0], res[1], res[2]);
            }
        }

        let mut new_q = BTreeSet::new();
        for (x, y) in q {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (xx, yy) = (x + dx, y + dy);
                // can be 'S' too!!
                if at(xx, yy) != '#' {
                    new_q.insert((xx, yy));
                }
            }
        }
        q = new_q;
        i += 1;
    }
}

fn part_two(lines: &str) -> i64 {
    let grid = string_to_grid(lines);
    // f(x) = c1 x^2 + c2 x + c3
    // f(0), f(1), f(2)
    let (a, b, c) = count(&grid);
    let c3 = a;
    let c1 = (c - 2 * b + a) / 2;
    let c2 = b - c1 - c3;
    let f = |x: i64| -> i64 { c1 * x * x + c2 * x + c3 };
    f(202300)
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
        panic!("couldn't read {}: {}", display, why);
    }

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines)); // 2s on release mode
}
