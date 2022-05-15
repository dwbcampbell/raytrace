use float_cmp::{ApproxEq, F64Margin};

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
        let result = Vec4::point(0.0, 0.0, 0.0);
        assert!(result.is_point());
        assert!(!result.is_vector());
    }
    #[test]
    fn is_vector() {
        let result = Vec4::vector(0.0, 0.0, 0.0);
        assert!(!result.is_point());
        assert!(result.is_vector());
    }

    #[test]
    fn make_point() {
        let p = Vec4::point(4.0, -4.0, 1.0);
        let v: Vec4 = Vec4 {
            x: 4.0,
            y: -4.0,
            z: 1.0,
            w: 1.0,
        };
        assert_eq!(p, v);
    }

    #[test]
    fn make_vector() {
        let p = Vec4::vector(4.0, -4.0, 1.0);
        let v: Vec4 = Vec4 {
            x: 4.0,
            y: -4.0,
            z: 1.0,
            w: 0.0,
        };
        assert_eq!(p, v);
    }
}
