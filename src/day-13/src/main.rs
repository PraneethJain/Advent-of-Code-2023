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

fn transpose_grid(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if grid.is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut transposed_grid = Vec::with_capacity(cols);
    for _ in 0..cols {
        transposed_grid.push(Vec::with_capacity(rows));
    }

    for i in 0..rows {
        for j in 0..cols {
            transposed_grid[j].push(grid[i][j]);
        }
    }

    transposed_grid
}

fn split_grid(grid: &Vec<Vec<char>>, i: usize) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut above = i;
    let mut below = i + 1;
    let mut above_grid: Vec<Vec<char>> = Vec::new();
    let mut below_grid: Vec<Vec<char>> = Vec::new();
    while below < grid.len() {
        above_grid.push(grid[above].clone());
        below_grid.push(grid[below].clone());
        if above == 0 {
            break;
        }
        above -= 1;
        below += 1;
    }
    (above_grid, below_grid)
}

fn count_differences(above_grid: &Vec<Vec<char>>, below_grid: &Vec<Vec<char>>) -> i32 {
    let mut differences = 0;
    for (above_vec, below_vec) in above_grid.iter().zip(below_grid.iter()) {
        for (&above_char, &below_char) in above_vec.iter().zip(below_vec.iter()) {
            if above_char != below_char {
                differences += 1;
            }
        }
    }
    differences
}

fn reflections(grid: &Vec<Vec<char>>, expected_differences: i32) -> i32 {
    for i in 0..(grid.len() - 1) {
        let (above_grid, below_grid) = split_grid(grid, i);
        if count_differences(&above_grid, &below_grid) == expected_differences {
            return i as i32 + 1;
        }
    }
    0
}

fn part_one(lines: &str) -> i32 {
    let grids: Vec<_> = lines.split("\n\n").map(string_to_grid).collect();
    let mut cols = 0;
    let mut rows = 0;
    for grid in grids {
        rows += reflections(&grid, 0);
        cols += reflections(&transpose_grid(grid), 0);
    }
    cols + 100 * rows
}

fn part_two(lines: &str) -> i32 {
    let grids: Vec<_> = lines.split("\n\n").map(string_to_grid).collect();
    let mut cols = 0;
    let mut rows = 0;
    for grid in grids {
        rows += reflections(&grid, 1);
        cols += reflections(&transpose_grid(grid), 1);
    }
    cols + 100 * rows
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
