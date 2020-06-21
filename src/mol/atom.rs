use super::{Element, Point3d};

#[derive(Debug)]
pub struct Atom {
    pub element: Element,
    pub position: Point3d,
    pub formal_charge: i32,
}

pub type AtomIndex = usize;

impl Atom {
    pub fn new(element: Element) -> Atom {
        Atom {
            element,
            position: Point3d::new(0.0, 0.0, 0.0),
            formal_charge: 0,
        }
    }

    pub fn from_atomic_number(atomic_number: u32) -> Atom {
        Atom::new(Element(atomic_number))
    }

    pub fn from_symbol(symbol: &str) -> Atom {
        Atom::new(Element::from_symbol(symbol))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_element() {
        let element = Element(12);
        let atom = Atom::new(element);
        
        assert_eq!(atom.element, Element(12));
        assert_eq!(atom.position, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(atom.formal_charge, 0);
    }

    #[test]
    fn new_from_atomic_number() {
        let atom = Atom::from_atomic_number(12);
        
        assert_eq!(atom.element, Element(12));
        assert_eq!(atom.position, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(atom.formal_charge, 0);
    }

    #[test]
    fn new_from_symbol() {
        let atom = Atom::from_symbol("C");
        
        assert_eq!(atom.element, Element(12));
        assert_eq!(atom.position, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(atom.formal_charge, 0);
    }
}