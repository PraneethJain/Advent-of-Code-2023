fn part_one(times: &[i64; 4], distances: &[i64; 4]) -> i64 {
    let mut res: i64 = 1;
    for (&t, &d) in times.iter().zip(distances) {
        let i = ((t * t - 4 * d) as f64).sqrt();
        let mut l = 0_f64.max((t as f64 - i) / 2.0).ceil() as i64;
        let mut r = (d as f64).min((t as f64 + i) / 2.0).floor() as i64;
        if l * (t - l) <= d {
            l += 1;
        }

        if r * (t - r) <= d {
            r -= 1;
        }
        res *= r - l + 1;
    }
    res
}

fn part_two(t: i64, d: i64) -> i64 {
    let i = ((t * t - 4 * d) as f64).sqrt();
    let mut l = 0_f64.max((t as f64 - i) / 2.0).ceil() as i64;
    let mut r = (d as f64).min((t as f64 + i) / 2.0).floor() as i64;
    if l * (t - l) <= d {
        l += 1;
    }

    if r * (t - r) <= d {
        r -= 1;
    }
    r - l + 1
}

fn main() {
    let times = [40, 82, 91, 66];
    let distances = [277, 1338, 1349, 1063];

    println!("{}", part_one(&times, &distances));
    println!("{}", part_two(40829166, 277133813491063));
}
