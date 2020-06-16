pub struct Element(u32);

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