use super::{AtomIndex, PropertyMap};

#[derive(Debug)]
pub struct Bond {
    pub from_atom_id: AtomIndex,
    pub to_atom_id: AtomIndex,
    pub properties: PropertyMap<BondProperty>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum BondProperty {
}

impl Bond {
    pub fn new(from_atom_id: AtomIndex, to_atom_id: AtomIndex) -> Bond {
        Bond {
            from_atom_id,
            to_atom_id,
            properties: PropertyMap::new(),
        }
    }
}