use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn combinations(record: String, nums: Vec<usize>) -> usize {
    if nums.is_empty() {
        return if record.contains('#') { 0 } else { 1 };
    }
    if nums.iter().any(|&x| x > record.len()) || record.is_empty() {
        return 0;
    }

    match record.chars().nth(0).unwrap() {
        '.' => combinations(record[1..].to_string(), nums),
        '#' => {
            if record[..nums[0]].chars().all(|x| "#?".contains(x))
                && (nums[0] == record.len() || ".?".contains(record.chars().nth(nums[0]).unwrap()))
            {
                combinations(
                    if nums[0] + 1 <= record.len() {
                        record[(nums[0] + 1)..].to_string()
                    } else {
                        String::new()
                    },
                    nums[1..].to_vec(),
                )
            } else {
                0
            }
        }
        '?' => {
            if record[..nums[0]].chars().all(|x| "#?".contains(x))
                && (nums[0] == record.len() || ".?".contains(record.chars().nth(nums[0]).unwrap()))
            {
                combinations(record[1..].to_string(), nums.clone())
                    + combinations(
                        if nums[0] + 1 <= record.len() {
                            record[(nums[0] + 1)..].to_string()
                        } else {
                            String::new()
                        },
                        nums[1..].to_vec(),
                    )
            } else {
                combinations(record[1..].to_string(), nums)
            }
        }
        c => panic!("found character {}", c),
    }
}

fn part_one(lines: &str) -> usize {
    lines
        .lines()
        .map(|line| {
            let (record, nums) = match line.split_once(' ') {
                Some((record, nums)) => (
                    record.trim(),
                    nums.trim()
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                ),
                None => panic!("no whitespace in line {}", line),
            };
            combinations(record.to_string(), nums)
        })
        .sum()
}

fn combinations_dp(
    record: String,
    nums: Vec<usize>,
    dp: &mut BTreeMap<(String, Vec<usize>), usize>,
) -> usize {
    match dp.get(&(record.clone(), nums.clone())) {
        Some(&res) => res,
        None => {
            let res = if nums.is_empty() {
                if record.contains('#') {
                    0
                } else {
                    1
                }
            } else if nums.iter().any(|&x| x > record.len()) || record.is_empty() {
                0
            } else {
                match record.chars().nth(0).unwrap() {
                    '.' => combinations_dp(record[1..].to_string(), nums.clone(), dp),
                    '#' => {
                        if record[..nums[0]].chars().all(|x| "#?".contains(x))
                            && (nums[0] == record.len()
                                || ".?".contains(record.chars().nth(nums[0]).unwrap()))
                        {
                            combinations_dp(
                                if nums[0] + 1 <= record.len() {
                                    record[(nums[0] + 1)..].to_string()
                                } else {
                                    String::new()
                                },
                                nums[1..].to_vec(),
                                dp,
                            )
                        } else {
                            0
                        }
                    }
                    '?' => {
                        if record[..nums[0]].chars().all(|x| "#?".contains(x))
                            && (nums[0] == record.len()
                                || ".?".contains(record.chars().nth(nums[0]).unwrap()))
                        {
                            combinations_dp(record[1..].to_string(), nums.clone(), dp)
                                + combinations_dp(
                                    if nums[0] + 1 <= record.len() {
                                        record[(nums[0] + 1)..].to_string()
                                    } else {
                                        String::new()
                                    },
                                    nums[1..].to_vec(),
                                    dp,
                                )
                        } else {
                            combinations_dp(record[1..].to_string(), nums.clone(), dp)
                        }
                    }
                    c => panic!("found character {}", c),
                }
            };
            dp.insert((record, nums), res);
            res
        }
    }
}

fn part_two(lines: &str) -> usize {
    let mut dp: BTreeMap<(String, Vec<usize>), usize> = BTreeMap::new();
    lines
        .lines()
        .map(|line| {
            let (record, nums) = match line.split_once(' ') {
                Some((record, nums)) => (
                    [record].repeat(5).join("?"),
                    nums.trim()
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                        .repeat(5),
                ),
                None => panic!("no whitespace in line {}", line),
            };
            combinations_dp(record.to_string(), nums.clone(), &mut dp)
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
    println!("{}", part_two(&lines));
}
