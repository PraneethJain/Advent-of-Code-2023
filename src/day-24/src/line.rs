use super::fraction::Fraction;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    m: Fraction,
    c: Fraction,
}

impl Line {
    pub fn new(m: Fraction, c: Fraction) -> Self {
        Self { m, c }
    }

    pub fn solve(&self, other: &Self) -> (Fraction, Fraction) {
        let x = (other.c - self.c) / (self.m - other.m);
        let y = self.m * x + self.c;
        (x, y)
    }
}
