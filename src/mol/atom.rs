use super::{Element, Point3d};

#[derive(Debug)]
pub struct Atom {
    pub element: Element,
    pub position: Point3d,
    pub formal_charge: i32,
    // pub isotope: u32
}