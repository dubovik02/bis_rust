use std::{fs::File, io, path::Path};

use bis_rust::{BinParser, CsvParser, Transaction, TransactionsParser, TxtParser};

fn main() -> io::Result<()> {
    let mut file_ok: bool = false; 

    let mut src_file_name = String::new();
    let mut trg_file_name = String::new();
    
    while !file_ok {
        src_file_name.clear();
        println!("Enter source file name or \'E\' to exit:");
        io::stdin().read_line(&mut src_file_name)?;    

        if src_file_name.trim() == "E" {
            return Ok(());
        }

        file_ok = Path::new(&src_file_name.trim()).exists();
        if !file_ok {
            println!("File {} does not exist", src_file_name.trim());
        }
    }


    file_ok = false;
    while !file_ok {
        trg_file_name.clear();
        println!("Enter target file name or \'E\' to exit:");
        io::stdin().read_line(&mut trg_file_name)?;    

        if trg_file_name.trim() == "E" {
            return Ok(());
        }

        file_ok = check_file_creation(&trg_file_name);

        if !file_ok {
            println!("File {} could'nt create", trg_file_name.trim());
        }
    }

    println!("-------------------------------------");
    println!("Source file: {}", src_file_name.trim());
    println!("Target file: {}", trg_file_name.trim());
    println!("-------------------------------------");

    let mut file = File::open(src_file_name.trim())?;
    let bin_parser: BinParser = BinParser::default();
    let res_vec: Vec<Transaction> = bin_parser.from_read(&mut file)?;

    println!("{}", res_vec.len());

    let mut out_file = File::create(Path::new(trg_file_name.trim()))?;
    bin_parser.write_to(&mut out_file, res_vec)?;

    println!("Press Enter to exit......");
    let mut key_pressed = String::new();
    io::stdin().read_line(&mut key_pressed)?;
    Ok(())
}

fn check_file_creation(path: &str) -> bool {
    match File::create(Path::new(path.trim())) {
        Ok(_) => true,
        Err(_) => false,
    }
}