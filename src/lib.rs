/*
polars-eda utility functions
*/

use polars::prelude::*;
use std::fs;
use std::io::Cursor;

//read in a csv file
pub fn read_csv(path: &str, headers: bool) -> Result<DataFrame, PolarsError> {
    let df = CsvReader::from_path(path).unwrap().has_header(headers).finish().unwrap();
    Ok(df)
}

//read in a json file
pub fn read_json(path: &str) -> Result<DataFrame, PolarsError> {
    // Read json file to string
    let json_str= fs::read_to_string(path).expect("Unable to read JSON");
    let df = JsonReader::new(Cursor::new(json_str)).finish().unwrap();
    Ok(df)
}

//summarise dataframe
pub fn df_summary(df: DataFrame) {
    println!("Dataframe Summary...");
    println!("{:?}", df.head(Some(5)));
    println!("{:?}", df.schema());
    println!("{:?}", df.describe(None));
}

// CI/CD test
#[cfg(test)]
mod tests {
    use crate::{read_csv,read_json};
    
    #[test]
    fn test_csv_reader() {
        let path = "./data/sample.csv";
        let df = read_csv(&path, true);
        assert!(df.is_ok());
    }
    #[test]
    fn test_json_reader() {
        let path = "./data/sample.json";
        let df = read_json(&path);
        assert!(df.is_ok());
    }
}