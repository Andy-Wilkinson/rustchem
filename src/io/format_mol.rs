use crate::mol::{Atom, Element, Molecule, Point3d};
use super::{FileReadError, ParseError, LineReader};
use super::utils::{parse_u32, parse_f64};

// Reference: https://web.archive.org/web/20070630061308/http:/www.mdl.com/downloads/public/ctfile/ctfile.pdf

pub fn read_mol(reader: impl std::io::Read) -> Result<Molecule, FileReadError> {
    let mut line_reader = LineReader::new(reader);
    let mut atoms: Vec<AtomSpec> = Vec::new();

    // TODO: Ignoring header
    line_reader.read_line()?;
    line_reader.read_line()?;
    line_reader.read_line()?;

    let counts_line = parse_counts(&line_reader.read_line()?).map_err(|source| FileReadError::LineParse { source, line: 3})?;

    for (line_number, atom_line) in line_reader.read_lines(counts_line.num_atoms).enumerate() {
        let atom = parse_atom_line(&atom_line?).map_err(|source| FileReadError::LineParse { source, line: line_number + 3 })?;
        atoms.push(atom)
    };

    for (line_number, atom_line) in line_reader.read_lines(counts_line.num_bonds).enumerate() {
        parse_bond_line(&atom_line?).map_err(|source| FileReadError::LineParse { source, line: counts_line.num_atoms as usize + line_number + 3 })?;
    };

    for _ in line_reader.read_lines(counts_line.num_atom_lists) {};
    for _ in line_reader.read_lines(counts_line.num_stext * 2) {};

    Ok(Molecule {
        atoms: atoms.into_iter().map(|spec| spec.to_atom()).collect::<Vec<_>>(),
    })
}

#[derive(Debug)]
struct CountsLine {
    pub num_atoms: u32,
    num_bonds: u32,
    num_atom_lists: u32,
    chiral_flag: bool,
    num_stext: u32,
    num_properties: u32,
    version: String,
}

#[derive(Debug)]
struct AtomSpec {
    pub element: Element,
    pub position: Point3d,
    pub formal_charge: i32,
}

fn parse_counts(line: &str) -> Result<CountsLine, ParseError> {
    let counts_line = CountsLine {
        num_atoms: parse_u32_default(&line[0..3], "atom count")?,
        num_bonds: parse_u32_default(&line[3..6], "bond count")?,
        num_atom_lists: parse_u32_default(&line[6..9], "atom list count")?,
        chiral_flag: parse_u32_default(&line[12..15], "chiral flag")? != 0,
        num_stext: parse_u32_default(&line[15..18], "stext count")?,
        num_properties: parse_u32_default(&line[30..33], "property list count")?,
        version: line[33..39].to_string(),
    };
    
    Ok(counts_line)
}

fn parse_atom_line(line: &str) -> Result<AtomSpec, ParseError> {
    let line = if line.len() >= 69 { line.to_string() } else { format!("{:69}", line) };

    let x = parse_f64(&line[0..10], "x-coordinate")?;
    let y = parse_f64(&line[10..20], "y-coordinate")?;
    let z = parse_f64(&line[20..30], "z-coordinate")?;
    let symbol = &line[31..34].trim();
    let _mass_difference = parse_u32_default(&line[34..36], "mass difference")?;
    let charge_id = parse_u32_default(&line[36..39], "charge")?;
    let _stereo_parity = parse_u32_default(&line[39..42], "atom stereo parity")?;
    let _hydrogen_count = parse_u32_default(&line[42..45], "hydrogen count")?;
    let _stereo_care_box = parse_u32_default(&line[45..48], "stereo care box")?;
    let _valence = parse_u32_default(&line[48..51], "valence")?;
    let _h0_designator = parse_u32_default(&line[51..54], "H0 designator")?;
    let _atom_mapping = parse_u32_default(&line[60..63], "atom-atom mapping number")?;
    let _inversion_flag = parse_u32_default(&line[63..66], "inversion/retention flag")?;
    let _exact_change_flag = parse_u32_default(&line[66..69], "inversion/retention flag")?;

    let charge = match charge_id {
        1 => 3, 2 => 2, 3 => 1,
        // TODO: 4 = doublet radical
        5 => -1,  6 => -2, 7 => -3,
        _ => 0
    };
    
    Ok(AtomSpec {
        element: Element::from_symbol(symbol),
        formal_charge: charge,
        position: Point3d::new(x, y, z)
    })
}

fn parse_bond_line(line: &str) -> Result<(), ParseError> {
    let line = if line.len() >= 21 { line.to_string() } else { format!("{:21}", line) };

    let _atom1 = parse_u32_default(&line[0..3], "atom 1")?;
    let _atom2 = parse_u32_default(&line[3..6], "atom 2")?;
    let _bond_type = parse_u32_default(&line[6..9], "bond type")?;
    let _bond_stereo = parse_u32_default(&line[9..12], "bond stereochemistry")?;
    let _bond_stereo = parse_u32_default(&line[15..18], "bond topology")?;
    let _reacting_center = parse_u32_default(&line[18..21], "reacting center status")?;

    println!("{} {}", _atom1, _atom2);
    
    Ok(())
}

fn parse_u32_default(val: &str, dest_nature: &str) -> Result<u32, ParseError> {
    if val.trim().len() == 0 { Ok(0) } else { parse_u32(val, dest_nature) }
}


impl AtomSpec {
    pub fn to_atom(self) -> Atom {
        return Atom {
            element: self.element,
            position: self.position,
            formal_charge: self.formal_charge
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_counts_okay() -> Result<(), ParseError> {
        let line = "  6  5  0  0  1  0              3 V2000";
        let counts_lines = parse_counts(&line)?;
        
        assert_eq!(counts_lines.num_atoms, 6);
        assert_eq!(counts_lines.num_bonds, 5);
        assert_eq!(counts_lines.num_atom_lists, 0);
        assert_eq!(counts_lines.chiral_flag, true);
        assert_eq!(counts_lines.num_stext, 0);
        assert_eq!(counts_lines.num_properties, 3);
        assert_eq!(counts_lines.version, " V2000");
        Ok(())
    }
}