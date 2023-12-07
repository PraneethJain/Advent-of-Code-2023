use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn create_frequency_map(input: &str) -> HashMap<char, usize> {
    let mut frequency_map: HashMap<char, usize> = HashMap::new();

    for character in input.chars() {
        let count = frequency_map.entry(character).or_insert(0);
        *count += 1;
    }

    frequency_map
}

fn part_one(lines: &str) -> i32 {
    let mut weights: Vec<(i32, &str, i32)> = vec![];
    for line in lines.lines() {
        let (hand, bid) = match line.split_once(' ') {
            Some((l, r)) => (
                l.trim(),
                match r.parse::<i32>() {
                    Ok(bid) => bid,
                    Err(why) => panic!("could not parse {}: {}", r, why),
                },
            ),
            None => panic!("invalid input"),
        };
        let map = create_frequency_map(hand);
        let counts: Vec<_> = map.values().map(|x| *x).collect();
        let score = match map.len() {
            1 => 7, // Five of a kind
            2 => {
                if counts.contains(&4) {
                    6 // Four of a kind
                } else {
                    5 // Full house
                }
            }
            3 => {
                if counts.iter().filter(|&x| *x == 2).count() == 2 {
                    3 // Two pair
                } else {
                    4 // Three of a kind
                }
            }
            4 => 2, // One pair
            5 => 1, // High Card
            _ => panic!("can't have more than 5 cards in a hand"),
        };
        weights.push((score, hand, bid));
    }

    let order = "AKQJT98765432";
    weights.sort_unstable_by(|&(w1, s1, _), &(w2, s2, _)| {
        let mut res = w1.cmp(&w2);
        if res == std::cmp::Ordering::Equal {
            for (c1, c2) in s1.chars().zip(s2.chars()) {
                let idx1 = order.find(c1).unwrap();
                let idx2 = order.find(c2).unwrap();
                if idx1 < idx2 {
                    res = std::cmp::Ordering::Greater;
                    break;
                } else if idx1 > idx2 {
                    res = std::cmp::Ordering::Less;
                    break;
                }
            }
        }
        res
    });
    weights
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, &(_, _, bid))| acc + (rank as i32 + 1) * bid)
}

fn part_two(lines: &str) -> i32 {
    let mut weights: Vec<(i32, &str, i32)> = vec![];
    for line in lines.lines() {
        let (hand, bid) = match line.split_once(' ') {
            Some((l, r)) => (
                l.trim(),
                match r.parse::<i32>() {
                    Ok(bid) => bid,
                    Err(why) => panic!("could not parse {}: {}", r, why),
                },
            ),
            None => panic!("invalid input"),
        };
        let map = create_frequency_map(hand);
        let counts: Vec<_> = map.values().map(|x| *x).collect();
        let joker = map.get(&'J');
        let score = match map.len() {
            1 => 7, // Five of a kind
            2 => {
                if counts.contains(&4) {
                    // Four of a kind
                    match joker {
                        Some(_) => 7, // Five of a kind
                        None => 6,
                    }
                } else {
                    // Full house
                    match joker {
                        Some(_) => 7, // Five of a kind
                        None => 5,
                    }
                }
            }
            3 => {
                if counts.iter().filter(|&x| *x == 2).count() == 2 {
                    // Two pair
                    match joker {
                        Some(&count) => match count {
                            1 => 5, // Full house
                            2 => 6, // Four of a kind
                            _ => panic!("not possible"),
                        },
                        None => 3,
                    }
                } else {
                    // Three of a kind
                    match joker {
                        Some(&count) => match count {
                            1 => 6, // Four of a kind
                            3 => 6, // Four of a kind
                            _ => panic!("not possible"),
                        },
                        None => 4,
                    }
                }
            }
            4 => match joker {
                // One pair
                Some(_) => 4, // Three of a kind
                None => 2,
            },
            5 => match joker {
                // High card
                Some(_) => 2, // One pair
                None => 1,
            },
            _ => panic!("can't have more than 5 cards in a hand"),
        };
        weights.push((score, hand, bid));
    }

    let order = "AKQT98765432J";
    weights.sort_unstable_by(|&(w1, s1, _), &(w2, s2, _)| {
        let mut res = w1.cmp(&w2);
        if res == std::cmp::Ordering::Equal {
            for (c1, c2) in s1.chars().zip(s2.chars()) {
                let idx1 = order.find(c1).unwrap();
                let idx2 = order.find(c2).unwrap();
                if idx1 < idx2 {
                    res = std::cmp::Ordering::Greater;
                    break;
                } else if idx1 > idx2 {
                    res = std::cmp::Ordering::Less;
                    break;
                }
            }
        }
        res
    });
    weights
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, &(_, _, bid))| acc + (rank as i32 + 1) * bid)
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
        _ => (),
    }

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
