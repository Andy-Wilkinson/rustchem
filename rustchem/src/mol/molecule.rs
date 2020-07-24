use super::{Atom, Bond, HasProperties, PropertyMap};

#[derive(Debug)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
    pub properties: PropertyMap<MoleculeProperty>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MoleculeProperty {
    Comment,
    CreationUser,
    CreationProgram,
    CreationDate,
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

impl Default for Molecule {
    fn default() -> Self {
        Molecule::new()
    }
}

impl HasProperties<MoleculeProperty> for Molecule {
    fn get_property_map(&self) -> &PropertyMap<MoleculeProperty> {
        &self.properties
    }

    fn get_property_map_mut(&mut self) -> &mut PropertyMap<MoleculeProperty> {
        &mut self.properties
    }
}
