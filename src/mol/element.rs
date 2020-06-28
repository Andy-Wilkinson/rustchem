use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;
use serde::Deserialize;
use super::MoleculeError;

lazy_static! {
    static ref ELEMENTS_BY_NUMBER: HashMap<u32, Element> = {
        let elements_config = include_str!("../../data_files/elements.csv").as_bytes();
        let mut config_reader = csv::Reader::from_reader(elements_config);

        let mut map: HashMap<u32, Element> = HashMap::new();

        for element in config_reader.deserialize() {
            let element: Element = element.expect("Unable to deserialize embedded elements.csv configuration file.");
            map.insert(element.atomic_number, element);
        }

        map
    };

    static ref ELEMENTS_BY_SYMBOL: HashMap<String, &'static Element> = {
        let mut map: HashMap<String, &'static Element> = HashMap::new();

        for element in ELEMENTS_BY_NUMBER.values() {
            map.insert(element.symbol.clone(), &element);
        }

        map
    };
}

#[derive(Deserialize)]
pub struct Element {
    pub atomic_number: u32,
    pub symbol: String,
}

impl Element {
    pub fn from_atomic_number(atomic_number: u32) -> Result<&'static Element, MoleculeError> {
        match ELEMENTS_BY_NUMBER.get(&atomic_number) {
            Some(element) => Ok(element),
            None => Err(MoleculeError::UnknownAtomicNumber(atomic_number))
        }
    }

    pub fn from_symbol(symbol: &str) -> Result<&'static Element, MoleculeError> {
        match ELEMENTS_BY_SYMBOL.get(symbol) {
            Some(element) => Ok(element),
            None => Err(MoleculeError::UnknownElementSymbol(symbol.to_string()))
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_number == other.atomic_number
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.symbol)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_atomic_number() -> Result<(), MoleculeError> {
        let element_c = Element::from_atomic_number(12)?;
        let element_o = Element::from_atomic_number(16)?;

        assert_eq!(element_c.atomic_number, 12);
        assert_eq!(element_o.atomic_number, 16);

        Ok(())
    }

    #[test]
    fn new_from_atomic_number_reads_properties() -> Result<(), MoleculeError> {
        let element_c = Element::from_atomic_number(12)?;

        assert_eq!(element_c.atomic_number, 12);
        assert_eq!(element_c.symbol, "C");

        Ok(())
    }

    #[test]
    fn new_from_atomic_number_error_unknown() -> Result<(), MoleculeError> {
        match Element::from_atomic_number(1234) {
            Err(MoleculeError::UnknownAtomicNumber(atomic_number)) => {
                assert_eq!(atomic_number, 1234);
            },
            _ => panic!("Expected MoleculeError::UnknownAtomicNumber") 
        }

        Ok(())
    }

    #[test]
    fn new_from_symbol() -> Result<(), MoleculeError> {
        let element_c = Element::from_symbol("C")?;
        let element_o = Element::from_symbol("O")?;

        assert_eq!(element_c.atomic_number, 12);
        assert_eq!(element_o.atomic_number, 16);

        Ok(())
    }

    #[test]
    fn new_from_symbol_error_unknown() -> Result<(), MoleculeError> {
        match Element::from_symbol("X") {
            Err(MoleculeError::UnknownElementSymbol(symbol)) => {
                assert_eq!(symbol, "X");
            },
            _ => panic!("Expected MoleculeError::UnknownElementSymbol") 
        }

        Ok(())
    }

    #[test]
    fn equality() -> Result<(), MoleculeError> {
        assert_eq!(Element::from_atomic_number(12)?, Element::from_atomic_number(12)?);
        assert_ne!(Element::from_atomic_number(14)?, Element::from_atomic_number(12)?);

        Ok(())
    }
}