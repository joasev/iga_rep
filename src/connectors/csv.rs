use csv::Reader;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::path::Path;



// Generic function to read a CSV file into a Vec of any type that implements DeserializeOwned
pub fn read<T: DeserializeOwned, P: AsRef<Path>>(file_path: P) -> Result<Vec<T>, Box<dyn Error>> {

    let file = File::open(file_path)?;

    let mut rdr = Reader::from_reader(file);
    
    // Collect deserialized records into a Vec<T>
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}



