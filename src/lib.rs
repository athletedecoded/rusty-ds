use polars::prelude::*;
use std::fs;
use std::io::Cursor;

// load a file from path and determine if csv/json
pub fn load_file(path: &str, headers: bool) -> Result<DataFrame, PolarsError> {
    println!("Loading file...");
    let file_type = path.split('.').last().unwrap();
    match file_type {
        "csv" => read_csv(path, headers),
        "json" => read_json(path),
        _ => panic!("File type not supported"),
    }
}

//read in a csv file
fn read_csv(path: &str, headers: bool) -> Result<DataFrame, PolarsError> {
    println!(".csv detected...");
    let df = CsvReader::from_path(path)
        .unwrap()
        .has_header(headers)
        .finish()
        .unwrap();
    Ok(df)
}

//read in a json file
fn read_json(path: &str) -> Result<DataFrame, PolarsError> {
    println!(".json detected...");
    // Read json file to string
    let json_str = fs::read_to_string(path).expect("Unable to read JSON");
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
    use crate::{read_csv, read_json};

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
