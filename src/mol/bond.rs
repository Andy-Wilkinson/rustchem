use super::AtomIndex;

#[derive(Debug)]
pub struct Bond {
    pub from_atom_id: AtomIndex,
    pub to_atom_id: AtomIndex,
}