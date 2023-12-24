use std::ops::{self, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    numerator: i128,
    denominator: i128,
}

impl Fraction {
    fn fix_sign(self) -> Self {
        if (self.numerator.is_negative() && self.denominator.is_negative())
            || (!self.numerator.is_negative() && self.denominator.is_negative())
        {
            Self {
                numerator: -self.numerator,
                denominator: -self.denominator,
            }
        } else {
            self
        }
    }

    pub fn new(numerator: i128, denominator: i128) -> Self {
        let g = gcd::binary_u128(numerator.unsigned_abs(), denominator.unsigned_abs()) as i128;
        Self {
            numerator: numerator / g,
            denominator: denominator / g,
        }
        .fix_sign()
    }

    #[allow(dead_code)]
    pub fn as_float(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl ops::Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denominator + self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl ops::Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denominator - self.denominator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl ops::Mul<i128> for Fraction {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        Fraction::new(self.numerator * rhs, self.denominator)
    }
}

impl ops::Div<i128> for Fraction {
    type Output = Self;

    fn div(self, rhs: i128) -> Self::Output {
        Fraction::new(self.numerator, self.denominator.mul(rhs))
    }
}

impl ops::Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.numerator * other.denominator).partial_cmp(&(self.denominator * other.numerator))
    }
}
