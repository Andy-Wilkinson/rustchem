use super::{Atom, Bond};

#[derive(Debug)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl Molecule {
    pub fn new() -> Molecule {
        Molecule {
            atoms: Vec::new(),
            bonds: Vec::new(),
        }
    }

    pub fn from_graph(atoms: Vec<Atom>, bonds: Vec<Bond>) -> Molecule {
        Molecule {
            atoms,
            bonds,
        }
    }
}