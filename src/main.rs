use std::fs::File;
use rustchem::io::read_mol;

fn main()
{
    let file = File::open("./test_files/alanine.mol");

    match file {
        Ok(file) => {
            match read_mol(file) {
                Ok(mol) => println!("{:#?}", mol),
                Err(_err) => panic!("Error parsing file"),
            }
        },
        Err(_err) => panic!("Error opening file"),
    }
}