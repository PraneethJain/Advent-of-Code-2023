use std::collections::BTreeSet;
use std::collections::VecDeque;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Ray {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn part_one(lines: &str) -> usize {
    let grid = string_to_grid(lines);
    print_grid(&grid);
    let mut q: VecDeque<Ray> = VecDeque::new();
    let mut seen: BTreeSet<Ray> = BTreeSet::new();
    let start = Ray {
        x: 0,
        y: -1,
        dx: 0,
        dy: 1,
    };
    q.push_back(start.clone());

    while !q.is_empty() {
        let mut ray = q.pop_front().expect("q is empty");
        ray.x += ray.dx;
        ray.y += ray.dy;

        if ray.x < 0 || ray.x >= grid.len() as i32 || ray.y < 0 || ray.y >= grid[0].len() as i32 {
            continue;
        }

        match grid[ray.x as usize][ray.y as usize] {
            '.' => {
                if seen.insert(ray.clone()) {
                    q.push_back(ray.clone());
                }
            }
            '-' => match ray.dy != 0 {
                true => {
                    if seen.insert(ray.clone()) {
                        q.push_back(ray.clone());
                    }
                }
                false => {
                    ray.dx = 0;
                    ray.dy = 1;
                    if seen.insert(ray.clone()) {
                        q.push_back(ray.clone());
                    }
                    ray.dy = -1;
                    if seen.insert(ray.clone()) {
                        q.push_back(ray.clone());
                    }
                }
            },
            '|' => match ray.dx != 0 {
                true => {
                    if seen.insert(ray.clone()) {
                        q.push_back(ray.clone());
                    }
                }
                false => {
                    ray.dy = 0;
                    ray.dx = 1;
                    if seen.insert(ray.clone()) {
                        q.push_back(ray.clone());
                    }
                    ray.dx = -1;
                    if seen.insert(ray.clone()) {
                        q.push_back(ray.clone());
                    }
                }
            },
            '/' => {
                (ray.dx, ray.dy) = (-ray.dy, -ray.dx);
                if seen.insert(ray.clone()) {
                    q.push_back(ray.clone());
                }
            }
            '\\' => {
                (ray.dx, ray.dy) = (ray.dy, ray.dx);
                if seen.insert(ray.clone()) {
                    q.push_back(ray.clone());
                }
            }
            x => panic!("found character {} in grid", x),
        }
    }

    let mut energized: BTreeSet<(i32, i32)> = BTreeSet::new();
    for ray in seen {
        energized.insert((ray.x, ray.y));
    }
    energized.iter().count()
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
