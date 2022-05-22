use float_cmp::{ApproxEq, F64Margin};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

const F64_MARGIN: F64Margin = F64Margin {
    epsilon: 0.0,
    ulps: 2,
};

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red.approx_eq(other.red, F64_MARGIN)
            && self.green.approx_eq(other.green, F64_MARGIN)
            && self.blue.approx_eq(other.blue, F64_MARGIN)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

// Scalar multiplication
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: rhs * self.red,
            green: rhs * self.green,
            blue: rhs * self.blue,
        }
    }
}

// Hadamard product
impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_construct() {
        let c = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_add() {
        let c1 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };
        let c2 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };
        let c3 = Color {
            red: 1.6,
            green: 0.7,
            blue: 1.0,
        };
        assert_eq!(c1 + c2, c3);
    }
}
