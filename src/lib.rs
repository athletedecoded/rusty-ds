use plotters::prelude::*;
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

pub fn plot_data(
    df: &DataFrame,
    x_col: &str,
    y_col: &str,
) {
    // get x and y columns --> transform to f64 Vec
    let x = df.column(x_col).unwrap().cast(&DataType::Float64).unwrap();
    let x_vec: Vec<f64> = x.f64().unwrap().into_no_null_iter().collect();
    let y = df.column(y_col).unwrap().cast(&DataType::Float64).unwrap();
    let y_vec: Vec<f64> = y.f64().unwrap().into_no_null_iter().collect();
    // Create (x,y) pairs
    let data: Vec<(f64, f64)> = x_vec.iter().zip(y_vec).map(|(x, y)| (*x, y)).collect();
    // Build plot
    let root = BitMapBackend::new("plots/scatter.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("EvCxR Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(0f64..250f64, 0f64..250f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();

    root.present().unwrap();
    println!("Plot saved to plots/scatter.png");

}

// CI/CD test
#[cfg(test)]
mod tests {
    use crate::{load_file, plot_data};
    #[test]
    fn load_csv() {
        let path = "./data/sample.csv";
        let df = load_file(&path, true);
        assert!(df.is_ok());
    }
    #[test]
    fn load_json() {
        let path = "./data/sample.json";
        let df = load_file(&path, false);
        assert!(df.is_ok());
    }

    #[test]
    fn test_plot() {
        let path = "./data/sample.csv";
        let df = load_file(&path, true).unwrap();
        plot_data(&df, "fats_g", "calories");
    }
}
