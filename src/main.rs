use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use structopt::StructOpt;

mod analyzer;
mod cli;
mod error;
mod logger;

use crate::analyzer::TypeAnalyzer;
use crate::cli::CliArgs;
use crate::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = CliArgs::from_args();
    logger::configure_logger(args.verbose);
    log::info!("Analysing file: {:?}", args.input_file_path);
    let file = File::open(args.input_file_path)?;
    let reader = BufReader::new(file);
    let mut analyzer = TypeAnalyzer::new();

    for line in reader.lines() {
        let line = line?;
        if let Err(e) = analyzer.check_line(line).await {
            log::warn!("Error while reading line: {}", e);
        }
    }

    analyzer.print_output();
    Ok(())
}
