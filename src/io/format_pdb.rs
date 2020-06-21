use std::io::{BufReader, BufRead};
use crate::mol::{Atom, Element, Molecule, Point3d};
use super::{FileReadError, ParseError};
use super::utils::{parse_u32, parse_i32, parse_f64};

pub fn read_pdb(reader: impl std::io::Read) -> Result<Molecule, FileReadError> {
    let reader = BufReader::new(reader).lines();
    let mut atoms: Vec<Atom> = Vec::new();

    for (count, line) in reader.enumerate() {
        let line = line?;
        let line = if line.len() >= 80 { line } else { format!("{:80}", line) };

        match &line[..6] {
            "      " => {},
            "ATOM  " | "HETATM" => {
                let atom = parse_pdb_atom(&line).map_err(|source| FileReadError::LineParse { source, line: count + 1})?;
                atoms.push(atom);
            },
            "TER   " => {},
            "CONECT" => {},
            "END   " => {},
            _ => panic!("Unknown identifier '{}'", &line[..6])
        };
    }

    let molecule = Molecule::from_graph(atoms, Vec::new());
    Ok(molecule)
}

fn parse_pdb_atom(line: &str) -> Result<Atom, ParseError> {
    let _serial = parse_u32(&line[6..11], "atom number")?;
    // let name = &line[12..16];
    // let alt_loc = &line[16..17];
    // let res_name = &line[17..20];
    // let chain_id = &line[21..22];
    // let res_seq = parse_u32(&line[22..26], "chain ID")?;
    // let i_code = &line[26..27];
    let x = parse_f64(&line[30..38], "x-coordinate")?;
    let y = parse_f64(&line[38..46], "y-coordinate")?;
    let z = parse_f64(&line[46..54], "z-coordinate")?;
    // let occupancy = parse_f64(&line[54..60], "occupancy")?;
    // let temp_factor = parse_f64(&line[60..66], "temperature factor")?;
    let element = &line[76..78].trim();
    let charge = &line[78..80];

    let charge = match &charge[1..2] {
        " " => 0,
        "+" => parse_i32(&charge[0..1], "charge")?,
        "-" => -parse_i32(&charge[0..1], "charge")?,
        _ => return Err(ParseError::Parse {name: "charge".to_string(), value: charge.to_string()})
    };

    Ok(Atom {
        element: Element::from_symbol(element),
        position: Point3d::new(x, y, z),
        formal_charge: charge,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_atom_atomic_element() -> Result<(), ParseError> {
        let line = "ATOM      4  CA  ALA L   1B     13.000  21.098  20.348  1.00 20.50      A    C  ";
        let atom = parse_pdb_atom(&line)?;
        assert_eq!(atom.element.get_atomic_number(), 12);
        Ok(())
    }

    #[test]
    fn parse_atom_position() -> Result<(), ParseError> {
        let line = "ATOM      4  CA  ALA L   1B     13.000  21.098  20.348  1.00 20.50      A    C  ";
        let atom = parse_pdb_atom(&line)?;
        assert_eq!(atom.position.x, 13.000);
        assert_eq!(atom.position.y, 21.098);
        assert_eq!(atom.position.z, 20.348);
        Ok(())
    }

    #[test]
    fn parse_atom_formal_charge_zero() -> Result<(), ParseError> {
        let line = "ATOM      4  CA  ALA L   1B     13.000  21.098  20.348  1.00 20.50      A    C  ";
        let atom = parse_pdb_atom(&line)?;
        assert_eq!(atom.formal_charge, 0);
        Ok(())
    }

    #[test]
    fn parse_atom_formal_charge_positive() -> Result<(), ParseError> {
        let line = "ATOM     47  NH1 ARG L   4       0.065   9.975  21.485  1.00  7.68      A    N1+";
        let atom = parse_pdb_atom(&line)?;
        assert_eq!(atom.formal_charge, 1);
        Ok(())
    }

    #[test]
    fn parse_atom_formal_charge_negative() -> Result<(), ParseError> {
        let line = "ATOM     17  OD2 ASP L   1A      7.250  19.552  18.526  0.50 22.65      A    O1-";
        let atom = parse_pdb_atom(&line)?;
        assert_eq!(atom.formal_charge, -1);
        Ok(())
    }
}