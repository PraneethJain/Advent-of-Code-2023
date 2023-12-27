mod fraction;
mod line;
mod vec3;

use fraction::Fraction;
use itertools::Itertools;
use line::Line;
use num_bigint::{BigInt, ToBigInt};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use vec3::Vec3;

fn part_one(lines: &str) -> usize {
    let valid = |&f: &Fraction, a: i128, b: i128| -> bool {
        f >= Fraction::new(200000000000000, 1)
            && f <= Fraction::new(400000000000000, 1)
            && (f / b - Fraction::new(a, b)) > Fraction::new(0, 1)
    };

    lines
        .lines()
        .map(|line| match line.split_once('@') {
            Some((l, r)) => {
                let position: Vec<_> = l
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect();
                let velocity: Vec<_> = r
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect();
                (
                    Line::new(
                        Fraction::new(velocity[1], velocity[0]),
                        Fraction::new(position[1], 1)
                            - Fraction::new(velocity[1], velocity[0]) * position[0],
                    ),
                    position[0],
                    position[1],
                    velocity[0],
                    velocity[1],
                )
            }
            None => panic!("no @ in line {}", line),
        })
        .combinations(2)
        .filter(|pair| {
            let (x, y) = pair[0].0.solve(&pair[1].0);
            [0, 1].iter().all(|&idx| {
                valid(&x, pair[idx].1, pair[idx].3) && valid(&y, pair[idx].2, pair[idx].4)
            })
        })
        .count()
}

fn part_two(lines: &str) -> BigInt {
    let stones: Vec<_> = lines
        .lines()
        .map(|line| match line.split_once('@') {
            Some((l, r)) => {
                let (x, y, z) = l
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let (vx, vy, vz) = r
                    .trim()
                    .split(',')
                    .map(|x| x.trim().parse::<i128>().unwrap())
                    .collect_tuple()
                    .unwrap();
                (Vec3::new(x, y, z), Vec3::new(vx, vy, vz))
            }
            None => panic!("no @ in line {}", line),
        })
        .collect();
    let (mut m, mut mx, mut my, mut mz) = (vec![], vec![], vec![], vec![]);
    [(0, 1), (2, 3), (4, 5)].iter().for_each(|&(i, j)| {
        let (p1, v1) = stones[i];
        let (p2, v2) = stones[j];

        let a = p2 - p1;
        let b = v2 - v1;
        // [a, p, b] = [p2, p1, v2] - [p2, p1, v1]
        // where [x, y, z] is scalar triple product
        let c4 = Vec3::box_prod(p2, p1, v2) - Vec3::box_prod(p2, p1, v1);
        let c1 = a.z * b.y - a.y * b.z;
        let c2 = a.x * b.z - a.z * b.x;
        let c3 = a.y * b.x - a.x * b.y;
        // c1 x + c2 y + c3 z = c4
        m.extend([c1, c2, c3]);
        mx.extend([c4, c2, c3]);
        my.extend([c1, c4, c3]);
        mz.extend([c1, c2, c4]);
    });

    let det = |mat: Vec<i128>| -> BigInt {
        let mat: Vec<BigInt> = mat.iter().map(|x| x.to_bigint().unwrap()).collect();
        &mat[0] * (&mat[4] * &mat[8] - &mat[5] * &mat[7])
            - &mat[1] * (&mat[3] * &mat[8] - &mat[5] * &mat[6])
            + &mat[2] * (&mat[3] * &mat[7] - &mat[4] * &mat[6])
    };

    // cramer's rule
    let d = det(m);
    let x = det(mx) / &d;
    let y = det(my) / &d;
    let z = det(mz) / &d;
    x + y + z
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
    println!("{}", part_two(&lines));
}
