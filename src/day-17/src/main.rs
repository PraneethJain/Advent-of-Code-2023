use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn string_to_grid(input: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = vec![vec![0; lines.get(0).map_or(0, |line| line.len())]; lines.len()];

    for (x, line) in lines.iter().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            grid[x][y] = ch.to_digit(10).expect("found non digit") as i32;
        }
    }

    grid
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    heat_loss: i32,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    same_dir_count: i32,
}

fn part_one(lines: &str) -> i32 {
    let grid = string_to_grid(lines);

    let mut heap: BinaryHeap<Point> = BinaryHeap::new();
    let mut seen: BTreeSet<(i32, i32, i32, i32, i32)> = BTreeSet::new();

    heap.push(Point {
        heat_loss: 0,
        x: 0,
        y: 0,
        dx: 0,
        dy: 0,
        same_dir_count: 0,
    });
    let mut first = true;

    while !heap.is_empty() {
        let point = heap.pop().expect("heap is empty");

        if point.x == grid.len() as i32 - 1 && point.y == grid[0].len() as i32 - 1 {
            return -point.heat_loss;
        }
        if !seen.insert((point.x, point.y, point.dx, point.dy, point.same_dir_count)) {
            continue;
        }

        // same direction
        if point.same_dir_count < 3 || first {
            first = false;
            let x = point.x + point.dx;
            let y = point.y + point.dy;
            if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
                heap.push(Point {
                    heat_loss: -(-point.heat_loss + grid[x as usize][y as usize]),
                    x,
                    y,
                    dx: point.dx,
                    dy: point.dy,
                    same_dir_count: point.same_dir_count + 1,
                })
            }
        }

        // different direction
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if (dx, dy) == (point.dx, point.dy) || (dx, dy) == (-point.dx, -point.dy) {
                continue;
            }
            let x = point.x + dx;
            let y = point.y + dy;
            if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
                heap.push(Point {
                    heat_loss: -(-point.heat_loss + grid[x as usize][y as usize]),
                    x,
                    y,
                    dx,
                    dy,
                    same_dir_count: 1,
                })
            }
        }
    }
    panic!("couldn't find path to end");
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
