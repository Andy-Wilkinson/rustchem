use std::fs::File;
use rustchem::io::format_pdb::read_pdb;

fn main()
{
    let file = File::open("../rustDock/test_files/1OYT-receptor.pdb");

    match file {
        Ok(file) => {
            match read_pdb(file) {
                Ok(mol) => println!("Success: {} atoms read", mol.atoms.len()),
                Err(_err) => panic!("Error parsing file"),
            }
        },
        Err(_err) => panic!("Error opening file"),
    }
}