use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part_one(lines: &str) -> usize {
    let mut result = 0;

    for (idx, line) in lines.lines().enumerate() {
        let parts = line.split(':').collect::<Vec<&str>>();
        let data = match parts.get(1) {
            Some(data) => data,
            None => panic!(),
        };

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for showing in data.split(';') {
            for color_data in showing.trim().split(',') {
                match color_data.trim().split_once(' ') {
                    Some((number_str, color)) => {
                        let num = match number_str.parse::<i32>() {
                            Ok(num) => num,
                            Err(_) => panic!("couldn't parse integer from {}", number_str),
                        };
                        match color {
                            "red" => red_max = red_max.max(num),
                            "green" => green_max = green_max.max(num),
                            "blue" => blue_max = blue_max.max(num),
                            _ => {}
                        }
                    }
                    None => (),
                }
            }
        }
        if red_max <= 12 && green_max <= 13 && blue_max <= 14 {
            result += idx + 1;
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
