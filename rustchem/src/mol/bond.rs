use super::{AtomIndex, PropertyMap};

#[derive(Debug)]
pub struct Bond {
    pub from_atom_id: AtomIndex,
    pub to_atom_id: AtomIndex,
    pub bond_type: BondType,
    pub properties: PropertyMap<BondProperty>,
}

#[derive(PartialEq, Debug)]
pub enum BondType {
    Covalent(i32),
    Aromatic,
    QueryList(Vec<BondType>),
    Any,
}

impl BondType {
    pub fn single() -> BondType {
        BondType::Covalent(1)
    }
    pub fn double() -> BondType {
        BondType::Covalent(2)
    }
    pub fn triple() -> BondType {
        BondType::Covalent(3)
    }
    pub fn single_or_double() -> BondType {
        BondType::QueryList(vec![BondType::Covalent(1), BondType::Covalent(2)])
    }
    pub fn single_or_aromatic() -> BondType {
        BondType::QueryList(vec![BondType::Covalent(1), BondType::Aromatic])
    }
    pub fn double_or_aromatic() -> BondType {
        BondType::QueryList(vec![BondType::Covalent(2), BondType::Aromatic])
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum BondProperty {}

impl Bond {
    pub fn new(from_atom_id: AtomIndex, to_atom_id: AtomIndex, bond_type: BondType) -> Bond {
        Bond {
            from_atom_id,
            to_atom_id,
            bond_type,
            properties: PropertyMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::MoleculeError;
    use super::*;

    #[test]
    fn new() -> Result<(), MoleculeError> {
        let bond = Bond::new(1, 2, BondType::single());

        assert_eq!(bond.from_atom_id, 1);
        assert_eq!(bond.to_atom_id, 2);
        assert_eq!(bond.bond_type, BondType::single());

        Ok(())
    }
}
