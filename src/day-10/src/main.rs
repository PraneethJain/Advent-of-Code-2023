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

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (x, row) in grid.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn part_one(lines: &str) -> i32 {
    let grid = string_to_grid(lines);

    let start = find_start(&grid).expect("couldn't find S in grid");
    let mut path: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
    path.insert(start);
    deq.push_back(start);

    while !deq.is_empty() {
        let (x, y) = deq.pop_front().expect("deque shouldn't be empty");
        if "S|JL".contains(grid[x][y])
            && "|7F".contains(grid[x - 1][y])
            && !path.contains(&(x - 1, y))
        {
            path.insert((x - 1, y));
            deq.push_back((x - 1, y));
        }

        if "S|7F".contains(grid[x][y])
            && "|JL".contains(grid[x + 1][y])
            && !path.contains(&(x + 1, y))
        {
            path.insert((x + 1, y));
            deq.push_back((x + 1, y));
        }

        if "S-J7".contains(grid[x][y])
            && "-LF".contains(grid[x][y - 1])
            && !path.contains(&(x, y - 1))
        {
            path.insert((x, y - 1));
            deq.push_back((x, y - 1));
        }

        if "S-LF".contains(grid[x][y])
            && "-J7".contains(grid[x][y + 1])
            && !path.contains(&(x, y + 1))
        {
            path.insert((x, y + 1));
            deq.push_back((x, y + 1));
        }
    }
    path.len() as i32 / 2
}

fn part_two(lines: &str) -> i32 {
    let mut grid = string_to_grid(lines);

    let start = find_start(&grid).expect("couldn't find S in grid");
    let mut path: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
    path.insert(start);
    deq.push_back(start);

    while !deq.is_empty() {
        let (x, y) = deq.pop_front().expect("deque shouldn't be empty");
        if "S|JL".contains(grid[x][y])
            && "|7F".contains(grid[x - 1][y])
            && !path.contains(&(x - 1, y))
        {
            path.insert((x - 1, y));
            deq.push_back((x - 1, y));
        }

        if "S|7F".contains(grid[x][y])
            && "|JL".contains(grid[x + 1][y])
            && !path.contains(&(x + 1, y))
        {
            path.insert((x + 1, y));
            deq.push_back((x + 1, y));
        }

        if "S-J7".contains(grid[x][y])
            && "-LF".contains(grid[x][y - 1])
            && !path.contains(&(x, y - 1))
        {
            path.insert((x, y - 1));
            deq.push_back((x, y - 1));
        }

        if "S-LF".contains(grid[x][y])
            && "-J7".contains(grid[x][y + 1])
            && !path.contains(&(x, y + 1))
        {
            path.insert((x, y + 1));
            deq.push_back((x, y + 1));
        }
    }

    grid[start.0][start.1] = '-';
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !path.contains(&(x, y)) {
                grid[x][y] = '.';
            }
        }
    }

    let mut outside_set: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut is_outside = true;
    for (x, row) in grid.iter().enumerate() {
        let mut dir_flag = false;
        for (y, &c) in row.iter().enumerate() {
            match c {
                '|' => is_outside = !is_outside,
                'L' | 'F' => dir_flag = c == 'L',
                '7' | 'J' => {
                    if c != if dir_flag { 'J' } else { '7' } {
                        is_outside = !is_outside;
                    }
                    dir_flag = false;
                }
                _ => {}
            }

            if is_outside {
                outside_set.insert((x, y));
            }
        }
    }
    (grid.len() * grid[0].len() - outside_set.union(&path).collect::<BTreeSet<_>>().len()) as i32
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
