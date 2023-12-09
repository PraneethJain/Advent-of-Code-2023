use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> i32 {
    let mut res = 0;

    for line in lines.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut num_of_nums = vec![nums];

        while !num_of_nums.last().unwrap().iter().all(|&x| x == 0) {
            num_of_nums.push(
                num_of_nums
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect(),
            )
        }

        loop {
            let last = num_of_nums.pop().unwrap().pop().unwrap();
            if num_of_nums.is_empty() {
                res += last;
                break;
            } else {
                *num_of_nums.last_mut().unwrap().last_mut().unwrap() += last;
            }
        }
    }

    res
}

fn part_two(lines: &str) -> i32 {
    let mut res = 0;

    for line in lines.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .rev()
            .collect();
        let mut num_of_nums = vec![nums];

        while !num_of_nums.last().unwrap().iter().all(|&x| x == 0) {
            num_of_nums.push(
                num_of_nums
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect(),
            )
        }

        loop {
            let last = num_of_nums.pop().unwrap().pop().unwrap();
            if num_of_nums.is_empty() {
                res += last;
                break;
            } else {
                *num_of_nums.last_mut().unwrap().last_mut().unwrap() += last;
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
    println!("{}", part_two(&lines));
}
