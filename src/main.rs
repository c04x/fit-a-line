mod approximations;

use approximations::least_squares;
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(help = "Separator to use", default_value = "\t", short, long)]
    separator: char,
    #[structopt(
        help = "Datapoints file path to use",
        default_value = "datapoints.txt",
        parse(from_os_str)
    )]
    file: std::path::PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    match least_squares(
        args.file.into_os_string().into_string().unwrap(),
        args.separator,
    ) {
        Ok(line) => println!(
            "A line that best fits given data points: y={0:.2}x{1:}{2:.2}",
            line.slope,
            if line.y_intercept > 0.0 { "+" } else { "" },
            line.y_intercept
        ),
        Err(e) => println!("{:?}", e),
    }
    Ok(())
}