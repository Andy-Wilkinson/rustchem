use super::ct_v2000::read_ct;
use super::utils::{parse_f64_default, parse_u32_default};
use crate::io::{FileReadError, LineReader, ParseError};
use crate::mol::{HasProperties, Molecule, MoleculeProperty};

pub fn read_mol(reader: impl std::io::Read) -> Result<Molecule, FileReadError> {
    let mut line_reader = LineReader::new(reader);

    // TODO: Ignoring header
    let molecule_name = line_reader.read_line()?;
    let header_line = parse_header(&line_reader.read_line()?)
        .map_err(|source| FileReadError::LineParse { source, line: 2 })?;
    let molecule_comment = line_reader.read_line()?;

    let mut molecule = read_ct(&mut line_reader)?;
    molecule.set_property(MoleculeProperty::Name, molecule_name);
    molecule.set_property(MoleculeProperty::Comment, molecule_comment);
    molecule.set_property(MoleculeProperty::CreationUser, header_line.user);
    molecule.set_property(MoleculeProperty::CreationProgram, header_line.program);
    molecule.set_property(MoleculeProperty::CreationDate, header_line.datetime);
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

    let line = if line.len() >= 52 {
        line.to_string()
    } else {
        format!("{:52}", line)
    };

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
    fn read_mol_alanine_header() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("./test_files/alanine_v2000.mol")?;
        let mol = read_mol(file)?;

        assert_eq!(
            mol.get_property_string(&MoleculeProperty::Name)?,
            Some("L-Alanine (13C)")
        );
        assert_eq!(
            mol.get_property_string(&MoleculeProperty::CreationUser)?,
            Some("GS")
        );
        assert_eq!(
            mol.get_property_string(&MoleculeProperty::CreationProgram)?,
            Some("MACCS-II")
        );
        assert_eq!(
            mol.get_property_string(&MoleculeProperty::CreationDate)?,
            Some("1016911536")
        );
        assert_eq!(
            mol.get_property_string(&MoleculeProperty::Comment)?,
            Some("Additional Comments")
        );
        Ok(())
    }
}
