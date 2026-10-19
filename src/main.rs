use csv; // parsing csv
use std::error::Error; // error handling

fn main() {

    // handle error in the main
    if let Err(e) = read_from_file("./month.csv") {
        eprintln!("{}", e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {

    // read csv file using the external crate
    let mut reader = csv::Reader::from_path(path)?;

    // log every result
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}