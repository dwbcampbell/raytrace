use float_cmp::{ApproxEq, F64Margin};
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x, F64_MARGIN)
            && self.y.approx_eq(other.y, F64_MARGIN)
            && self.z.approx_eq(other.z, F64_MARGIN)
            && self.w.approx_eq(other.w, F64_MARGIN)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
            w: rhs * self.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

const POINT_W: f64 = 1.0;
const VECTOR_W: f64 = 0.0;
const F64_MARGIN: F64Margin = F64Margin {
    epsilon: 0.0,
    ulps: 2,
};

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: POINT_W,
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: VECTOR_W,
        }
    }

    pub fn is_point(&self) -> bool {
        self.w.approx_eq(POINT_W, F64_MARGIN)
    }

    pub fn is_vector(&self) -> bool {
        self.w.approx_eq(VECTOR_W, F64_MARGIN)
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;
    #[test]
    fn is_point() {
        // A tuple with w=1.0 is a point
        let result = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(result.x, 4.3);
        assert_eq!(result.y, -4.2);
        assert_eq!(result.z, 3.1);
        assert_eq!(result.w, 1.0);
        assert!(result.is_point());
        assert!(!result.is_vector());
    }
    #[test]
    fn is_vector() {
        // A tuple with w=0 is a vector
        let result = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(result.x, 4.3);
        assert_eq!(result.y, -4.2);
        assert_eq!(result.z, 3.1);
        assert_eq!(result.w, 0.0);
        assert!(!result.is_point());
        assert!(result.is_vector());
    }

    #[test]
    fn make_point() {
        // point() creates tuples with w=1
        let p = Tuple::point(4.0, -4.0, 3.0);
        let v: Tuple = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(p, v);
    }

    #[test]
    fn make_vector() {
        // vector() creates tuples with w=0
        let p = Tuple::vector(4.0, -4.0, 1.0);
        let v: Tuple = Tuple {
            x: 4.0,
            y: -4.0,
            z: 1.0,
            w: 0.0,
        };
        assert_eq!(p, v);
    }

    /*
     * Adding Tuples
     */

    #[test]
    fn add_vectors() {
        // Adding two vectors
        let a1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };

        let a2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        let result = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };

        assert_eq!(a1 + a2, result);
    }

    /*
     * Subtracting Tuples
     */

    #[test]
    fn sub_two_points() {
        // Subtracting two points
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let result = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, result);
    }

    #[test]
    fn sub_vector_from_point() {
        // Subtracting a vector from a point
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        let result = Tuple::point(-2.0, -4.0, -6.0);
        assert_eq!(p - v, result);
    }

    #[test]
    fn sub_two_vectors() {
        // Subtracting two vectors
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        let result = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(v1 - v2, result);
    }

    /*
     * Negating Tuples
     */

    #[test]
    fn sub_from_zero() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            -a,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    /*
     * Scalar Multiplication and Division
     */
    #[test]
    fn test_mult_by_scalar() {
        // Multiplying a tuple by a scalar
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let result = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq!(a * 3.5, result);
    }
    #[test]
    fn test_mult_by_fraction() {
        // Multiplying a tuple by a fraction
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let result = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(a * 0.5, result);
    }

    #[test]
    fn test_div_by_scalar() {
        // Dividing a tuple by a scalar
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let result = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(a / 2.0, result);
    }
}
