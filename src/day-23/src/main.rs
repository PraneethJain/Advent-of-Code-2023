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

fn dfs(grid: &Vec<Vec<char>>, cur: (i32, i32), par: (i32, i32)) -> i32 {
    let valid = |pos: (i32, i32)| -> bool {
        pos.0 >= 0 && pos.0 < grid.len() as i32 && pos.1 >= 0 && pos.1 < grid[0].len() as i32
    };

    let mut res = 0;
    let dirs = match grid[cur.0 as usize][cur.1 as usize] {
        '.' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        c => panic!("found character {c}"),
    };

    for (dx, dy) in dirs {
        let (nx, ny) = (cur.0 + dx, cur.1 + dy);
        if (nx, ny) != par && valid((nx, ny)) {
            match grid[nx as usize][ny as usize] {
                '#' => (),
                _ => res = res.max(1 + dfs(grid, (nx, ny), cur)),
            }
        }
    }
    res
}

fn part_one(lines: &str) -> i32 {
    let grid = string_to_grid(lines);
    dfs(&grid, (0, 1), (0, 0))
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
