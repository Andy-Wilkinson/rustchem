#[derive(PartialEq, Debug)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Point3d {
        Point3d { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_f64_eq;

    #[test]
    fn new_from_coordinates() {
        let point = Point3d::new(1.0, 2.0, 3.5);

        assert_f64_eq(point.x, 1.0);
        assert_f64_eq(point.y, 2.0);
        assert_f64_eq(point.z, 3.5);
    }

    #[test]
    fn equality() {
        assert_eq!(Point3d::new(1.0, 2.0, 3.5), Point3d::new(1.0, 2.0, 3.5));
        assert_ne!(Point3d::new(1.2, 2.0, 3.5), Point3d::new(1.0, 2.0, 3.5));
        assert_ne!(Point3d::new(1.0, 2.3, 3.5), Point3d::new(1.0, 2.0, 3.5));
        assert_ne!(Point3d::new(1.0, 2.0, 3.1), Point3d::new(1.0, 2.0, 3.5));
    }
}
