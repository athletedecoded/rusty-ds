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
    after_help = "Example: cargo run plot --path <path/to/data> --headers <true/false> --x <column_name> --y <column_name>"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Summary {
        #[clap(long)]
        path: String,
        #[clap(long)]
        headers: bool,
    },
    // Plot {
    //     #[clap(long)]
    //     path: String,
    //     #[clap(long)]
    //     headers: bool,
    //     #[clap(long)]
    //     x: String,
    //     #[clap(long)]
    //     y: String,
    // },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Summary { path, headers }) => {
            let df = rusty_ds::load_file(&path, headers);
            match df {
                Ok(df) => rusty_ds::df_summary(df),
                Err(e) => println!("Error: {}", e),
            }
        }
        // Some(Commands::Plot {
        //     path,
        //     headers,
        //     x,
        //     y,
        // }) => {
        //     let df = rusty_ds::load_file(&path, headers);
        //     match df {
        //         Ok(df) => rusty_ds::plot(df, &x, &y),
        //         Err(e) => println!("Error: {}", e),
        //     }
        // }
        None => {
            println!("No command specified");
            return;
        }
    };
}
