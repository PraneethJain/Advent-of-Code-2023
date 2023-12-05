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

fn at(grid: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    if x < grid.len() && y < grid[0].len() {
        grid[x][y]
    } else {
        '.'
    }
}

fn is_symbol(ch: char) -> bool {
    !ch.is_digit(10) && ch != '.'
}

fn surrounded(grid: &Vec<Vec<char>>, x: usize, y_start: usize, y_end: usize) -> i32 {
    let mut has_symbol = false;

    let extremes = match y_start {
        0 => vec![y_end],
        y_start => vec![y_start - 1, y_end],
    };
    for y in extremes {
        if x > 0 {
            has_symbol |= is_symbol(at(grid, x - 1, y));
        }
        has_symbol |= is_symbol(at(grid, x, y));
        has_symbol |= is_symbol(at(grid, x + 1, y));
    }

    for y in y_start..y_end {
        if x > 0 {
            has_symbol |= is_symbol(at(grid, x - 1, y));
        }
        has_symbol |= is_symbol(at(grid, x + 1, y));
    }

    match has_symbol {
        true => {
            let num_str: String = grid[x][y_start..y_end].iter().collect();
            match num_str.parse::<i32>() {
                Ok(res) => res,
                Err(why) => panic!("Error parsing integer: {}", why),
            }
        }
        false => 0,
    }
}

fn part_one(lines: &str) -> i32 {
    let mut result = 0;

    let grid = string_to_grid(lines);
    // print_grid(&grid);

    for x in 0..grid.len() {
        let mut y = 0;
        while y < grid[x].len() {
            let mut j = 0;
            while y + j < grid[x].len() && grid[x][y + j].is_digit(10) {
                j += 1;
            }
            y += match j {
                0 => 1,
                j => {
                    result += surrounded(&grid, x, y, y + j);
                    j
                }
            }
        }
    }

    result
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
