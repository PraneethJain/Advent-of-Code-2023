use std::collections::BTreeSet;
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

fn part_one(lines: &str) -> i32 {
    let grid = string_to_grid(lines);
    let empty_rows: BTreeSet<_> = grid
        .iter()
        .enumerate()
        .filter_map(|(x, vec)| {
            if vec.iter().all(|&c| c == '.') {
                Some(x)
            } else {
                None
            }
        })
        .collect();
    let empty_columns: BTreeSet<_> = (0..grid[0].len())
        .filter_map(|y| {
            if grid.iter().all(|vec| vec[y] == '.') {
                Some(y)
            } else {
                None
            }
        })
        .collect();
    let galaxy_positions: BTreeSet<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(x, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(move |(y, &c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect();

    let mut res = 0;
    for (i, &(x1, y1)) in galaxy_positions.iter().enumerate() {
        for &(x2, y2) in galaxy_positions.iter().skip(i + 1) {
            let (startx, endx) = (x1.min(x2), x1.max(x2));
            let (starty, endy) = (y1.min(y2), y1.max(y2));
            for x in startx..endx {
                res += if empty_rows.contains(&x) { 2 } else { 1 }
            }
            for y in starty..endy {
                res += if empty_columns.contains(&y) { 2 } else { 1 }
            }
        }
    }
    res
}

fn part_two(lines: &str) -> i64 {
    let grid = string_to_grid(lines);
    let empty_rows: BTreeSet<_> = grid
        .iter()
        .enumerate()
        .filter_map(|(x, vec)| {
            if vec.iter().all(|&c| c == '.') {
                Some(x)
            } else {
                None
            }
        })
        .collect();
    let empty_columns: BTreeSet<_> = (0..grid[0].len())
        .filter_map(|y| {
            if grid.iter().all(|vec| vec[y] == '.') {
                Some(y)
            } else {
                None
            }
        })
        .collect();
    let galaxy_positions: BTreeSet<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(x, vec)| {
            vec.iter()
                .enumerate()
                .filter_map(move |(y, &c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect();

    let mut res = 0;
    for (i, &(x1, y1)) in galaxy_positions.iter().enumerate() {
        for &(x2, y2) in galaxy_positions.iter().skip(i + 1) {
            let (startx, endx) = (x1.min(x2), x1.max(x2));
            let (starty, endy) = (y1.min(y2), y1.max(y2));
            for x in startx..endx {
                res += if empty_rows.contains(&x) { 1000000 } else { 1 }
            }
            for y in starty..endy {
                res += if empty_columns.contains(&y) {
                    1000000
                } else {
                    1
                }
            }
        }
    }
    res
}
fn main() {
    let path = Path::new("input.txt");

    let mut file = File::open(&path).expect("couldn't open file");
    let mut lines = String::new();
    file.read_to_string(&mut lines).expect("couldn't read file");

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
