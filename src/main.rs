use std::error::Error;
use std::io;

mod domcsv;
use domcsv::DomDay;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for result in reader.deserialize() {
        let domday: DomDay = result?;
        println!("{}", domday);
    }
    Ok(())
}
