use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type Brick = ((usize, usize, usize), (usize, usize, usize));

#[allow(dead_code)]
fn print_xz(bricks: &Vec<Brick>) {
    let (x_max, z_max) = bricks.iter().fold((0, 0), |acc, &cur| {
        (
            acc.0.max(cur.0 .0).max(cur.1 .0),
            acc.1.max(cur.0 .2).max(cur.1 .2),
        )
    });

    let mut grid = vec![vec!['.'; x_max + 1]; z_max + 1];
    for (idx, (start, end)) in bricks.iter().enumerate() {
        for x in start.0..=end.0 {
            grid[start.2][x] = char::from_digit(idx as u32, 10).unwrap();
        }
        for z in start.2..=end.2 {
            grid[z][start.0] = char::from_digit(idx as u32, 10).unwrap();
        }
    }

    for row in grid.iter().rev() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_yz(bricks: &Vec<Brick>) {
    let (y_max, z_max) = bricks.iter().fold((0, 0), |acc, &cur| {
        (
            acc.0.max(cur.0 .1).max(cur.1 .1),
            acc.1.max(cur.0 .2).max(cur.1 .2),
        )
    });

    let mut grid = vec![vec!['.'; y_max + 1]; z_max + 1];
    for (idx, (start, end)) in bricks.iter().enumerate() {
        for y in start.1..=end.1 {
            grid[start.2][y] = char::from_digit(idx as u32, 10).unwrap();
        }
        for z in start.2..=end.2 {
            grid[z][start.1] = char::from_digit(idx as u32, 10).unwrap();
        }
    }

    for row in grid.iter().rev() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn stabilize(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut max_heights: HashMap<(usize, usize), usize> = HashMap::new();
    let mut stabilized = bricks.clone();
    for (idx, brick) in bricks.iter().enumerate() {
        let mut can_fall_to = 1;
        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                match max_heights.get(&(x, y)) {
                    Some(&z) => can_fall_to = can_fall_to.max(z),
                    None => (),
                }
            }
        }
        let (minz, maxz) = (brick.0 .2.min(brick.1 .2), brick.0 .2.max(brick.1 .2));
        stabilized[idx].0 .2 = can_fall_to;
        stabilized[idx].1 .2 = can_fall_to + maxz - minz;
        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                max_heights.insert((x, y), stabilized[idx].1 .2 + 1);
            }
        }
    }
    stabilized
}

fn part_one(lines: &str) -> i32 {
    let mut bricks: Vec<_> = lines
        .lines()
        .map(|line| match line.split_once('~') {
            Some((l, r)) => {
                let start: Vec<usize> = l.trim().split(',').map(|x| x.parse().unwrap()).collect();
                let end: Vec<usize> = r.trim().split(',').map(|x| x.parse().unwrap()).collect();
                ((start[0], start[1], start[2]), (end[0], end[1], end[2]))
            }
            None => panic!("no ~ in line {}", line),
        })
        .collect();
    bricks.sort_unstable_by_key(|brick| brick.0 .2.min(brick.1 .2));
    let stabilized = stabilize(&bricks);

    stabilized
        .iter()
        .map(|brick| {
            let removed: Vec<_> = stabilized
                .clone()
                .iter()
                .filter_map(|x| if x == brick { None } else { Some(x.to_owned()) })
                .collect();
            if removed == stabilize(&removed) {
                1
            } else {
                0
            }
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
