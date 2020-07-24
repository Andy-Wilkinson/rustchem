use super::utils::{parse_f64, parse_i32, parse_u32, parse_usize};
use super::{FileReadError, LineReader, ParseError};
use crate::mol::{Atom, Bond, BondType, HasProperties, Molecule, MoleculeProperty, Point3d};

// Reference: https://web.archive.org/web/20070630061308/http:/www.mdl.com/downloads/public/ctfile/ctfile.pdf

pub fn read_mol(reader: impl std::io::Read) -> Result<Molecule, FileReadError> {
    let mut line_reader = LineReader::new(reader);

    // TODO: Ignoring header
    let molecule_name = line_reader.read_line()?;
    let header_line = parse_header(&line_reader.read_line()?)
        .map_err(|source| FileReadError::LineParse { source, line: 2 })?;
    let molecule_comment = line_reader.read_line()?;

    let counts_line = parse_counts(&line_reader.read_line()?)
        .map_err(|source| FileReadError::LineParse { source, line: 3 })?;

    let mut atoms = line_reader
        .read_lines(counts_line.num_atoms)
        .enumerate()
        .map(|(line_number, atom_line)| {
            parse_atom_line(&atom_line?).map_err(|source| FileReadError::LineParse {
                source,
                line: line_number + 3,
            })
        })
        .collect::<Result<Vec<Atom>, FileReadError>>()?;

    let bonds = line_reader
        .read_lines(counts_line.num_bonds)
        .enumerate()
        .map(|(line_number, bond_line)| {
            parse_bond_line(&bond_line?).map_err(|source| FileReadError::LineParse {
                source,
                line: counts_line.num_atoms as usize + line_number + 3,
            })
        })
        .collect::<Result<Vec<Bond>, FileReadError>>()?;

    for _ in line_reader.read_lines(counts_line.num_atom_lists) {}
    for _ in line_reader.read_lines(counts_line.num_stext * 2) {}

    let mut has_charge_props = false;

    loop {
        let line = line_reader.read_line()?;
        match &line[..6] {
            "M  END" => break,
            "M  CHG" => {
                if !has_charge_props {
                    reset_atom_charges(&mut atoms);
                    has_charge_props = true
                };
            }
            "M  RAD" => {
                if !has_charge_props {
                    reset_atom_charges(&mut atoms);
                    has_charge_props = true
                };
            }
            _ => {}
        }
    }

    let mut molecule = Molecule::from_graph(atoms, bonds);
    molecule
        .properties
        .set_property(MoleculeProperty::Name, molecule_name);
    molecule
        .properties
        .set_property(MoleculeProperty::Comment, molecule_comment);
    molecule
        .properties
        .set_property(MoleculeProperty::CreationUser, header_line.user);
    molecule
        .properties
        .set_property(MoleculeProperty::CreationProgram, header_line.program);
    molecule
        .properties
        .set_property(MoleculeProperty::CreationDate, header_line.datetime);
    Ok(molecule)
}

#[derive(Debug)]
struct HeaderLine {
    pub user: String,
    pub program: String,
    pub datetime: String,
    pub flag_3d: bool,
    pub scaling_int: u32,
    pub scaling_float: f64,
    pub energy: f64,
    pub reg_number: u32,
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
    if val.trim().is_empty() {
        Ok(0)
    } else {
        parse_u32(val, dest_nature)
    }
}

fn parse_i32_default(val: &str, dest_nature: &str) -> Result<i32, ParseError> {
    if val.trim().is_empty() {
        Ok(0)
    } else {
        parse_i32(val, dest_nature)
    }
}

fn parse_f64_default(val: &str, dest_nature: &str) -> Result<f64, ParseError> {
    if val.trim().is_empty() {
        Ok(0.0)
    } else {
        parse_f64(val, dest_nature)
    }
}

fn parse_usize_default(val: &str, dest_nature: &str) -> Result<usize, ParseError> {
    if val.trim().is_empty() {
        Ok(0)
    } else {
        parse_usize(val, dest_nature)
    }
}

fn parse_header(line: &str) -> Result<HeaderLine, ParseError> {
    /*
    Header Line: 'IIPPPPPPPPMMDDYYHHmmddSSssssssssssEEEEEEEEEEEERRRRRR'

    II = creation user's initials
    PPPPPPPP = creation program name
    MMDDYYHHmm = date/time
    dd = dimensional codes
    SS = scaling factor (Integer)
    ssssssssss = scaling factor (Float ####.#####)
    EEEEEEEEEEEE = energy if from modelling program output (Float ######.#####)
    RRRRRR = internal registry number from MDL form (Integer)
    */

    Ok(HeaderLine {
        user: line[0..2].to_string(),
        program: line[2..10].to_string(),
        datetime: line[10..20].to_string(),
        flag_3d: &line[20..22] == "3D",
        scaling_int: parse_u32_default(&line[22..24], "scaling factor")?,
        scaling_float: parse_f64_default(&line[24..34], "scaling factor")?,
        energy: parse_f64_default(&line[34..46], "energy")?,
        reg_number: parse_u32_default(&line[46..52], "registration number")?,
    })
}

fn parse_counts(line: &str) -> Result<CountsLine, ParseError> {
    /*
    Counts Line: 'aaabbblllfffcccsssxxxrrrpppiiimmmvvvvvv'

    aaa = number of atoms (maximum 255 for some software)
    bbb = number of bonds (maximum 255 for some software)
    lll = number of atom lists (maximum of 30 for some software)
    fff = (obsolete)
    ccc = chiral flag (1=chiral, 0=achiral)
    sss = number of stext entries
    xxx = (obsolete)
    rrr = (obsolete)
    ppp = (obsolete)
    iii = (obsolete)
    mmm = number of additional property lines including 'M END' (now obselete and assigned to 999)
    vvvvvv = version string (' V2000' or ' V3000')
    */

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
    /*
    Atom Block: 'xxxxx.xxxxyyyyy.yyyyzzzzz.zzzz aaaddcccssshhhbbbvvvHHHrrriiimmmnnneee'

    xxxxx.xxxx = x-coordinate
    yyyyy.yyyy = y-coordinate
    zzzzz.zzzz = z-coordinate
    aaa = atom symbol (or 'L'=atom list, 'A'/'Q'/'*'=unspecified, 'LP'=lone pair, 'R#'=R-group)
    dd = mass difference from default isotope (range -3..4, outside of limits are zero, 'M ISO' takes precedence)
    ccc = charge(0=uncharged, 1=+3, 2=+2, 3=+1, 5=-1, 6=-2, 7=-3, 4=doublet radical, 'M CHG/RAD' take precedence)
    sss = atom stero-parity (0=not stereo, 1=odd, 2=even, 3=either/unmarked, Ignored when read)
    hhh (*) = hydrogen count + 1 (1=H0 (no H-atoms allowed unless drawn), 2..5=Hn ('n' or more H-atoms + those drawn)
    bbb (*) = stereo care box (for double bond stereochemistry, 0=ignore, 1=must match drawn, NB: must be 1 at both ends of bond)
    vvv = valence (0=unspecified, 1..14=number of bonds to atom including implied H-atoms, 15=zero valence)
    HHH = H0 designator (redundant with 'hhh' and not used)
    rrr = (not used)
    iii = (not used)
    mmm = atom-atom mapping number (for reactions, 1..number of atoms)
    nnn = inversion/retention flag (for reactions, 0=not applied, 1=inversion, 2=retention)
    eee (*) = exact change flag (for reactions, 0=not applied, 1=change on atom must be exactly as shown)

    NB: (*) specifies properties that apply only to queries
    */
    let line = if line.len() >= 69 {
        line.to_string()
    } else {
        format!("{:69}", line)
    };

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
        1 => 3,
        2 => 2,
        3 => 1,
        // TODO: 4 = doublet radical
        5 => -1,
        6 => -2,
        7 => -3,
        _ => 0,
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
    /*
    Bond Block: '111222tttsssxxxrrrccc'

    111 = first atom index (starting from 1)
    222 = second atom index (starting from 1)
    ttt = bond type (1=single, 2=double, 3=triple; (*) 4=aromatic, 5=single/double, 6=single/aromatic, 7=double/aromatic, 8=any)
    sss = bond stereo (Single bonds 0=none, 1=up, 4=either, 6=down; Double bonds 0=as drawn, 3=either cis/trans)
    xxx = (not used)
    rrr (*) = bond topology (0=either, 1=ring, 2=chain)
    ccc (*) = reacting centre status (0=unmarked, 1=center, -1=not center, 2=no change, 4=made/broken, 8=bond order change, 4+8, 4+1, 8+1, 12+1)

    NB: (*) specifies properties that apply only to queries
    */

    let line = if line.len() >= 21 {
        line.to_string()
    } else {
        format!("{:21}", line)
    };

    let from_atom_id = parse_usize_default(&line[0..3], "atom 1")?;
    let to_atom_id = parse_usize_default(&line[3..6], "atom 2")?;
    let bond_type = parse_u32_default(&line[6..9], "bond type")?;
    let _bond_stereo = parse_u32_default(&line[9..12], "bond stereochemistry")?;
    let _bond_topology = parse_u32_default(&line[15..18], "bond topology")?;
    let _reacting_center = parse_u32_default(&line[18..21], "reacting center status")?;

    let bond_type = match bond_type {
        1 => BondType::single(),
        2 => BondType::double(),
        3 => BondType::triple(),
        4 => BondType::Aromatic,
        5 => BondType::single_or_double(),
        6 => BondType::single_or_aromatic(),
        7 => BondType::double_or_aromatic(),
        8 => BondType::Any,
        _ => {
            return Err(ParseError::InvalidValue {
                name: "bond type".to_string(),
                value: line[6..9].to_string(),
            })
        }
    };

    Ok(Bond::new(from_atom_id - 1, to_atom_id - 1, bond_type))
}

pub fn reset_atom_charges(atoms: &mut Vec<Atom>) {
    for atom in atoms {
        atom.formal_charge = 0;
        // TODO: atom.radical = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_f64_eq;
    use std::fs::File;

    #[test]
    fn parse_header_line() -> Result<(), ParseError> {
        let line = "GSMACCS-II10169115362D 1   0.00366     0.00123    42";
        let header_line = parse_header(&line)?;

        assert_eq!(header_line.user, "GS");
        assert_eq!(header_line.program, "MACCS-II");
        assert_eq!(header_line.datetime, "1016911536");
        assert_eq!(header_line.flag_3d, false);
        assert_eq!(header_line.scaling_int, 1);
        assert_f64_eq(header_line.scaling_float, 0.00366);
        assert_f64_eq(header_line.energy, 0.00123);
        assert_eq!(header_line.reg_number, 42);
        Ok(())
    }

    #[test]
    fn parse_counts_line() -> Result<(), ParseError> {
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

    #[test]
    fn parse_bond_standard() -> Result<(), ParseError> {
        let line = "  2  5  2  0  0  0";
        let bond = parse_bond_line(&line)?;

        assert_eq!(bond.from_atom_id, 1);
        assert_eq!(bond.to_atom_id, 4);
        assert_eq!(bond.bond_type, BondType::double());
        assert_eq!(bond.properties.len(), 0);

        Ok(())
    }

    #[test]
    fn parse_bond_type_simple() -> Result<(), ParseError> {
        let line_single = "  2  5  1  0  0  0";
        let line_double = "  2  5  2  0  0  0";
        let line_triple = "  2  5  3  0  0  0";
        let line_aromatic = "  2  5  4  0  0  0";

        let bond_single = parse_bond_line(&line_single)?;
        let bond_double = parse_bond_line(&line_double)?;
        let bond_triple = parse_bond_line(&line_triple)?;
        let bond_aromatic = parse_bond_line(&line_aromatic)?;

        assert_eq!(bond_single.bond_type, BondType::single());
        assert_eq!(bond_double.bond_type, BondType::double());
        assert_eq!(bond_triple.bond_type, BondType::triple());
        assert_eq!(bond_aromatic.bond_type, BondType::Aromatic);

        Ok(())
    }

    #[test]
    fn parse_bond_type_query() -> Result<(), ParseError> {
        let line_singleordouble = "  2  5  5  0  0  0";
        let line_singleoraromatic = "  2  5  6  0  0  0";
        let line_doubleoraromatic = "  2  5  7  0  0  0";
        let line_any = "  2  5  8  0  0  0";

        let bond_singleordouble = parse_bond_line(&line_singleordouble)?;
        let bond_singleoraromatic = parse_bond_line(&line_singleoraromatic)?;
        let bond_doubleoraromatic = parse_bond_line(&line_doubleoraromatic)?;
        let bond_any = parse_bond_line(&line_any)?;

        assert_eq!(bond_singleordouble.bond_type, BondType::single_or_double());
        assert_eq!(
            bond_singleoraromatic.bond_type,
            BondType::single_or_aromatic()
        );
        assert_eq!(
            bond_doubleoraromatic.bond_type,
            BondType::double_or_aromatic()
        );
        assert_eq!(bond_any.bond_type, BondType::Any);

        Ok(())
    }

    #[test]
    fn parse_bond_type_error_invalid() -> Result<(), ParseError> {
        let line_bond_zero = "  2  5  0  0  0  0";
        let line_bond_nine = "  2  5  9  0  0  0";

        match parse_bond_line(&line_bond_zero) {
            Err(ParseError::InvalidValue { name, value }) => {
                assert_eq!(name, "bond type");
                assert_eq!(value, "  0");
            }
            _ => panic!("Expected ParseError::Parse"),
        }

        match parse_bond_line(&line_bond_nine) {
            Err(ParseError::InvalidValue { name, value }) => {
                assert_eq!(name, "bond type");
                assert_eq!(value, "  9");
            }
            _ => panic!("Expected ParseError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn read_mol_alanine_header() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("./test_files/alanine.mol")?;
        let mol = read_mol(file)?;

        assert_eq!(
            mol.properties.get_string(&MoleculeProperty::Name)?,
            Some("L-Alanine (13C)")
        );
        assert_eq!(
            mol.properties.get_string(&MoleculeProperty::CreationUser)?,
            Some("GS")
        );
        assert_eq!(
            mol.properties
                .get_string(&MoleculeProperty::CreationProgram)?,
            Some("MACCS-II")
        );
        assert_eq!(
            mol.properties.get_string(&MoleculeProperty::CreationDate)?,
            Some("1016911536")
        );
        assert_eq!(
            mol.properties.get_string(&MoleculeProperty::Comment)?,
            Some("Additional Comments")
        );
        Ok(())
    }
}
