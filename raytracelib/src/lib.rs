use float_cmp::{ApproxEq, F64Margin};
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug)]
pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x, F64_MARGIN)
            && self.y.approx_eq(other.y, F64_MARGIN)
            && self.z.approx_eq(other.z, F64_MARGIN)
            && self.w.approx_eq(other.w, F64_MARGIN)
    }
}

impl Add for Vec4 {
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

impl Sub for Vec4 {
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

impl Neg for Vec4 {
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

impl Mul<f64> for Vec4 {
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

impl Div<f64> for Vec4 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: rhs / self.x,
            y: rhs / self.y,
            z: rhs / self.z,
            w: rhs / self.w,
        }
    }
}

const POINT_W: f64 = 1.0;
const VECTOR_W: f64 = 0.0;
const F64_MARGIN: F64Margin = F64Margin {
    epsilon: 0.0,
    ulps: 2,
};

impl Vec4 {
    pub fn point(x: f64, y: f64, z: f64) -> Vec4 {
        Vec4 {
            x,
            y,
            z,
            w: POINT_W,
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Vec4 {
        Vec4 {
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
    use super::Vec4;
    #[test]
    fn is_point() {
        // A tuple with w=1.0 is a point
        let result = Vec4 {
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
        let result = Vec4 {
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
        let p = Vec4::point(4.0, -4.0, 3.0);
        let v: Vec4 = Vec4 {
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
        let p = Vec4::vector(4.0, -4.0, 1.0);
        let v: Vec4 = Vec4 {
            x: 4.0,
            y: -4.0,
            z: 1.0,
            w: 0.0,
        };
        assert_eq!(p, v);
    }

    /*
     * Adding 
     */

    #[test]
    fn add_vectors() {
        // Adding two vectors
        let a1 = Vec4 {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        
        let a2 = Vec4 {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        let result = Vec4 {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };
             
        assert_eq!(a1 + a2, result);
    }

    /*
     * Subtracting
     */

    #[test]
    fn sub_two_points() {
        // Subtracting two points
        let p1 = Vec4::point(3.0,2.0,1.0);
        let p2 = Vec4::point(5.0,6.0,7.0);
        let result = Vec4::vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, result);
    }

    #[test]
    fn sub_vector_from_point() {
        // Subtracting a vector from a point
        let p = Vec4::point(3.0,2.0,1.0);
        let v = Vec4::vector(5.0,6.0,7.0);
        let result = Vec4::point(-2.0, -4.0, -6.0);
        assert_eq!(p - v, result);
    }

    #[test]
    fn sub_two_vectors() {
        // Subtracting two vectors
        let v1 = Vec4::vector(3.0,2.0,1.0);
        let v2 = Vec4::vector(5.0,6.0,7.0);
        let result = Vec4::vector(-2.0, -4.0, -6.0);
        assert_eq!(v1 - v2, result);
    } 
}
