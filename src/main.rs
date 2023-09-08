/*
A Polars EDA tool for .csv and .json datafiles
*/

use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "A CLI tool for EDA using Polars",
    after_help = "Example: cargo run csv --path <path/to/data.csv> --has_headers"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Csv {
        #[clap(long)]
        path: String,
        #[clap(long)]
        headers: bool,
    },
    Json {
        #[clap(long)]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();
    let df = match args.command {
        Some(Commands::Csv { path, headers }) => {
            println!("Loading CSV to dataframe...");
            rusty_ds::read_csv(&path, headers)
        }
        Some(Commands::Json { path }) => {
            println!("Loading JSON to dataframe...");
            rusty_ds::read_json(&path)
        }
        None => {
            println!("No command specified");
            return;
        }
    };
    match df {
        Ok(df) => rusty_ds::df_summary(df),
        Err(e) => println!("Error: {:?}", e),
    }
}
