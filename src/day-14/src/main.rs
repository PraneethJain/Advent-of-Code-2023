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

fn transpose_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn part_one(lines: &str) -> usize {
    let grid = string_to_grid(lines);
    transpose_grid(
        &transpose_grid(&grid)
            .iter()
            .map(|vec| {
                vec.split(|&c| c == '#')
                    .map(|part| {
                        let mut part_vec = part.to_vec();
                        part_vec.sort_unstable();
                        part_vec.iter().rev().collect()
                    })
                    .collect::<Vec<String>>()
                    .join("#")
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<_>>(),
    )
    .iter()
    .enumerate()
    .fold(0, |acc, (idx, row)| {
        acc + row.iter().filter(|&&x| x == 'O').count() * (grid.len() - idx)
    })
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
