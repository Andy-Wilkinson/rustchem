use crate::mol::{Atom, Bond, Molecule, Point3d};
use super::{FileReadError, ParseError, LineReader};
use super::utils::{parse_u32, parse_i32, parse_usize, parse_f64};

// Reference: https://web.archive.org/web/20070630061308/http:/www.mdl.com/downloads/public/ctfile/ctfile.pdf

pub fn read_mol(reader: impl std::io::Read) -> Result<Molecule, FileReadError> {
    let mut line_reader = LineReader::new(reader);
    let mut atoms: Vec<Atom> = Vec::new();
    let mut bonds: Vec<Bond> = Vec::new();

    // TODO: Ignoring header
    line_reader.read_line()?;
    line_reader.read_line()?;
    line_reader.read_line()?;

    let counts_line = parse_counts(&line_reader.read_line()?).map_err(|source| FileReadError::LineParse { source, line: 3})?;

    for (line_number, atom_line) in line_reader.read_lines(counts_line.num_atoms).enumerate() {
        let atom = parse_atom_line(&atom_line?).map_err(|source| FileReadError::LineParse { source, line: line_number + 3 })?;
        atoms.push(atom);
    };

    for (line_number, bond_line) in line_reader.read_lines(counts_line.num_bonds).enumerate() {
        let bond = parse_bond_line(&bond_line?).map_err(|source| FileReadError::LineParse { source, line: counts_line.num_atoms as usize + line_number + 3 })?;
        bonds.push(bond);
    };

    for _ in line_reader.read_lines(counts_line.num_atom_lists) {};
    for _ in line_reader.read_lines(counts_line.num_stext * 2) {};

    let mut has_charge_props = false;

    loop {
        let line = line_reader.read_line()?;
        match &line[..6] {
            "M  END" => break,
            "M  CHG" => {
                if !has_charge_props { reset_atom_charges(&mut atoms); has_charge_props = true };
            },
            "M  RAD" => {
                if !has_charge_props { reset_atom_charges(&mut atoms); has_charge_props = true };
            },
            _ => {},
        }
    };

    let molecule = Molecule::from_graph(atoms, bonds);
    Ok(molecule)
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

fn parse_u32_default(val: &str, dest_nature: &str) -> Result<u32, ParseError> {
    if val.trim().len() == 0 { Ok(0) } else { parse_u32(val, dest_nature) }
}

fn parse_i32_default(val: &str, dest_nature: &str) -> Result<i32, ParseError> {
    if val.trim().len() == 0 { Ok(0) } else { parse_i32(val, dest_nature) }
}

fn parse_usize_default(val: &str, dest_nature: &str) -> Result<usize, ParseError> {
    if val.trim().len() == 0 { Ok(0) } else { parse_usize(val, dest_nature) }
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

pub fn parse_atom_line(line: &str) -> Result<Atom, ParseError> {
    let line = if line.len() >= 69 { line.to_string() } else { format!("{:69}", line) };

    let x = parse_f64(&line[0..10], "x-coordinate")?;
    let y = parse_f64(&line[10..20], "y-coordinate")?;
    let z = parse_f64(&line[20..30], "z-coordinate")?;
    let symbol = &line[31..34].trim();
    let mass_difference = parse_i32_default(&line[34..36], "mass difference")?;
    let charge_id = parse_u32_default(&line[36..39], "charge")?;
    let _stereo_parity = parse_u32_default(&line[39..42], "atom stereo parity")?;
    let _hydrogen_count = parse_u32_default(&line[42..45], "hydrogen count")?;
    let _stereo_care_box = parse_u32_default(&line[45..48], "stereo care box")?;
    let _valence = parse_u32_default(&line[48..51], "valence")?;
    let _h0_designator = parse_u32_default(&line[51..54], "H0 designator")?;
    let _atom_mapping = parse_u32_default(&line[60..63], "atom-atom mapping number")?;
    let _inversion_flag = parse_u32_default(&line[63..66], "inversion/retention flag")?;
    let _exact_change_flag = parse_u32_default(&line[66..69], "inversion/retention flag")?;

    let formal_charge = match charge_id {
        1 => 3, 2 => 2, 3 => 1,
        // TODO: 4 = doublet radical
        5 => -1,  6 => -2, 7 => -3,
        _ => 0
    };
    
    let mut atom = Atom::from_symbol(symbol)?;
    atom.formal_charge = formal_charge;
    atom.position = Point3d::new(x, y, z);

    if mass_difference >= -3 && mass_difference <= 3 && mass_difference != 0 {
        atom.isotope = Some(((atom.element.most_common_isotope) as i32 + mass_difference) as u32);
    }

    Ok(atom)
}

pub fn parse_bond_line(line: &str) -> Result<Bond, ParseError> {
    let line = if line.len() >= 21 { line.to_string() } else { format!("{:21}", line) };

    let from_atom_id = parse_usize_default(&line[0..3], "atom 1")?;
    let to_atom_id = parse_usize_default(&line[3..6], "atom 2")?;
    let _bond_type = parse_u32_default(&line[6..9], "bond type")?;
    let _bond_stereo = parse_u32_default(&line[9..12], "bond stereochemistry")?;
    let _bond_stereo = parse_u32_default(&line[15..18], "bond topology")?;
    let _reacting_center = parse_u32_default(&line[18..21], "reacting center status")?;

    
    Ok(Bond::new(from_atom_id - 1, to_atom_id - 1))
}

pub fn reset_atom_charges(atoms: &mut Vec<Atom>) {
    for i in 0..atoms.len() {
        atoms[i].formal_charge = 0;
        // TODO: atom.radical = 0;
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

    #[test]
    fn parse_atom_standard() -> Result<(), ParseError> {
        let line = "   -0.6622    0.5342    0.0000 C   0  0  2  0  0  0";
        let atom = parse_atom_line(&line)?;

        assert_eq!(atom.element.atomic_number, 12);
        assert_eq!(atom.position, Point3d::new(-0.6622, 0.5342, 0.0000));
        assert_eq!(atom.formal_charge, 0);
        assert_eq!(atom.isotope, None);
        assert_eq!(atom.properties.len(), 0);

        Ok(())
    }

    #[test]
    fn parse_atom_charged() -> Result<(), ParseError> {
        let line_pos3 = "   -0.6622    0.5342    0.0000 C   0  1  2  0  0  0";
        let line_pos2 = "   -0.6622    0.5342    0.0000 C   0  2  2  0  0  0";
        let line_pos1 = "   -0.6622    0.5342    0.0000 C   0  3  2  0  0  0";
        let line_neg1 = "   -0.6622    0.5342    0.0000 C   0  5  2  0  0  0";
        let line_neg2 = "   -0.6622    0.5342    0.0000 C   0  6  2  0  0  0";
        let line_neg3 = "   -0.6622    0.5342    0.0000 C   0  7  2  0  0  0";

        let atom_pos3 = parse_atom_line(&line_pos3)?;
        let atom_pos2 = parse_atom_line(&line_pos2)?;
        let atom_pos1 = parse_atom_line(&line_pos1)?;
        let atom_neg1 = parse_atom_line(&line_neg1)?;
        let atom_neg2 = parse_atom_line(&line_neg2)?;
        let atom_neg3 = parse_atom_line(&line_neg3)?;

        assert_eq!(atom_pos3.formal_charge, 3);
        assert_eq!(atom_pos2.formal_charge, 2);
        assert_eq!(atom_pos1.formal_charge, 1);
        assert_eq!(atom_neg1.formal_charge, -1);
        assert_eq!(atom_neg2.formal_charge, -2);
        assert_eq!(atom_neg3.formal_charge, -3);

        Ok(())
    }

    #[test]
    fn parse_atom_isotope() -> Result<(), ParseError> {
        let line_c13 = "   -0.6622    0.5342    0.0000 C   1  0  2  0  0  0";
        let line_c14 = "   -0.6622    0.5342    0.0000 C   2  0  2  0  0  0";
        let line_c11 = "   -0.6622    0.5342    0.0000 C  -1  0  2  0  0  0";
        let line_n15 = "   -0.6622    0.5342    0.0000 N   1  0  2  0  0  0";

        let atom_c13 = parse_atom_line(&line_c13)?;
        let atom_c14 = parse_atom_line(&line_c14)?;
        let atom_c11 = parse_atom_line(&line_c11)?;
        let atom_n15 = parse_atom_line(&line_n15)?;

        assert_eq!(atom_c13.isotope, Some(13));
        assert_eq!(atom_c14.isotope, Some(14));
        assert_eq!(atom_c11.isotope, Some(11));
        assert_eq!(atom_n15.isotope, Some(15));

        Ok(())
    }

    #[test]
    fn parse_atom_isotope_outofrange() -> Result<(), ParseError> {
        let line_1 = "   -0.6622    0.5342    0.0000 C  -4  0  2  0  0  0";
        let line_2 = "   -0.6622    0.5342    0.0000 C   4  0  2  0  0  0";
        let line_3 = "   -0.6622    0.5342    0.0000 C   0  0  2  0  0  0";

        let atom_1 = parse_atom_line(&line_1)?;
        let atom_2 = parse_atom_line(&line_2)?;
        let atom_3 = parse_atom_line(&line_3)?;

        assert_eq!(atom_1.isotope, None);
        assert_eq!(atom_2.isotope, None);
        assert_eq!(atom_3.isotope, None);

        Ok(())
    }
}