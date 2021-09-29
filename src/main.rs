use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use structopt::StructOpt;

mod analyzer;
mod error;
mod cli;

use crate::analyzer::TypeAnalyzer;
use crate::error::AppError;
use crate::cli::CliArgs;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = CliArgs::from_args();
    println!("{:?}", args);
    let file = File::open(args.input_file_path)?;
    let reader = BufReader::new(file);
    let mut analyzer = TypeAnalyzer::new();

    for line in reader.lines() {
        let line = line?;
        analyzer.check_line(line).await.expect("Add error handling");
    }

    analyzer.print_output();
    Ok(())
}
