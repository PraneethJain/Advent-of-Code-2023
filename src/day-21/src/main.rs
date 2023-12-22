use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = vec![vec![' '; lines.get(0).map_or(0, |line| line.len())]; lines.len()];

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

fn find_start(grid: &Vec<Vec<char>>) -> Option<(i32, i32)> {
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
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)]  {
                let (xx, yy) = (x + dx, y + dy);
                if xx > 0 && xx < grid.len() as i32 && yy > 0 && yy < grid[0].len() as i32 {
                    if grid[xx as usize][yy as usize] == '.' && !seen.contains(&(xx, yy)) {
                        seen.insert((xx, yy));
                        q.push_back((xx, yy, steps - 1));
                    }
                }
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
