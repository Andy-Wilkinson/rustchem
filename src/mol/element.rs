#[derive(PartialEq, Debug)]
pub struct Element(pub u32);

impl Element {
    pub fn get_atomic_number(&self) -> u32 {
        return self.0;
    }

    pub fn from_symbol(symbol: &str) -> Element {
        let atomic_number = match symbol {
            "C" => 12,
            "N" => 14,
            "O" => 16,
            _ => 0
        };
        Element(atomic_number)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_symbol() {
        let element_c = Element::from_symbol("C");
        let element_o = Element::from_symbol("O");

        assert_eq!(element_c.0, 12);
        assert_eq!(element_o.0, 16);
    }

    #[test]
    fn equality() {
        assert_eq!(Element(12), Element(12));
        assert_ne!(Element(14), Element(12));
        assert_ne!(Element(0), Element(12));
    }
}