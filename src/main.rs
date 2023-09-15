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
    Plot {
        #[clap(long)]
        path: String,
        #[clap(long)]
        headers: bool,
        #[clap(long)]
        x: String,
        #[clap(long)]
        y: String,
    },
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
        Some(Commands::Plot {
            path,
            headers,
            x,
            y,
        }) => {
            let df = rusty_ds::load_file(&path, headers);
            match df {
                Ok(df) => {
                    // get data
                    let data = rusty_ds::zip_data(&df, &x,&y);
                    // get x and y limits
                    let xlims = rusty_ds::get_lims(&df, &x);
                    let ylims = rusty_ds::get_lims(&df, &y);
                    // plot data
                    rusty_ds::plot_data(data, xlims, ylims);
                },
                Err(e) => println!("Error: {}", e),
            }
        }
        None => {
            println!("No command specified");
        }
    };
}
