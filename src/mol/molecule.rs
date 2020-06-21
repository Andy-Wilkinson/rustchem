use super::{Atom, Bond, PropertyMap};

#[derive(Debug)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub properties: PropertyMap<MoleculeProperty>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MoleculeProperty {
    Name,
}

impl Molecule {
    pub fn new() -> Molecule {
        Molecule {
            atoms: Vec::new(),
            bonds: Vec::new(),
            properties: PropertyMap::new(),
        }
    }

    pub fn from_graph(atoms: Vec<Atom>, bonds: Vec<Bond>) -> Molecule {
        Molecule {
            atoms,
            bonds,
            properties: PropertyMap::new(),
        }
    }
}