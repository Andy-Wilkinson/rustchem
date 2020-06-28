use super::{Element, Point3d, PropertyMap, MoleculeError};

#[derive(Debug)]
pub struct Atom {
    pub element: &'static Element,
    pub position: Point3d,
    pub formal_charge: i32,
    pub properties: PropertyMap<AtomProperty>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AtomProperty {
    PartialCharge,
}

pub type AtomIndex = usize;

impl Atom {
    pub fn new(element: &'static Element) -> Atom {
        Atom {
            element,
            position: Point3d::new(0.0, 0.0, 0.0),
            formal_charge: 0,
            properties: PropertyMap::new(),
        }
    }

    pub fn from_atomic_number(atomic_number: u32) -> Result<Atom, MoleculeError> {
        Ok(Atom::new(Element::from_atomic_number(atomic_number)?))
    }

    pub fn from_symbol(symbol: &str) -> Result<Atom, MoleculeError> {
        Ok(Atom::new(Element::from_symbol(symbol)?))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_element() -> Result<(), MoleculeError> {
        let element = Element::from_atomic_number(12)?;
        let atom = Atom::new(element);
        
        assert_eq!(atom.element.atomic_number, 12);
        assert_eq!(atom.position, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(atom.formal_charge, 0);
        assert_eq!(atom.properties.len(), 0);

        Ok(())
    }

    #[test]
    fn new_from_atomic_number() -> Result<(), MoleculeError> {
        let atom = Atom::from_atomic_number(12)?;
        
        assert_eq!(atom.element.atomic_number, 12);
        assert_eq!(atom.position, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(atom.formal_charge, 0);
        assert_eq!(atom.properties.len(), 0);

        Ok(())
    }

    #[test]
    fn new_from_atomic_number_error_unknown() -> Result<(), MoleculeError> {
        match Atom::from_atomic_number(1234) {
            Err(MoleculeError::UnknownAtomicNumber(atomic_number)) => {
                assert_eq!(atomic_number, 1234);
            },
            _ => panic!("Expected MoleculeError::UnknownAtomicNumber") 
        }

        Ok(())
    }

    #[test]
    fn new_from_symbol() -> Result<(), MoleculeError> {
        let atom = Atom::from_symbol("C")?;
        
        assert_eq!(atom.element.atomic_number, 12);
        assert_eq!(atom.position, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(atom.formal_charge, 0);
        assert_eq!(atom.properties.len(), 0);

        Ok(())
    }

    #[test]
    fn new_from_symbol_error_unknown() -> Result<(), MoleculeError> {
        match Atom::from_symbol("X") {
            Err(MoleculeError::UnknownElementSymbol(symbol)) => {
                assert_eq!(symbol, "X");
            },
            _ => panic!("Expected MoleculeError::UnknownElementSymbol") 
        }

        Ok(())
    }
}