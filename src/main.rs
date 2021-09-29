use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio_stream::StreamExt;

mod analyzer;
mod cli;
mod error;
mod logger;

use crate::analyzer::TypeAnalyzer;
use crate::cli::CliArgs;
use crate::error::AppError;

async fn run(input_file_path: PathBuf) -> Result<(), AppError> {
    log::info!("Analysing file: {:?}", input_file_path);

    // TODO: This could be async
    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);
    let mut analyzer = TypeAnalyzer::new();

    let mut stream = tokio_stream::iter(reader.lines().enumerate());
    while let Some((no, line)) = stream.next().await {
        let _res = analyzer
            .check_line(line?)
            .await
            .map_err(|err| log::warn!("Error while reading line no {}: {}", no, err));
    }

    log::info!("Final result:\n");
    analyzer.print_output();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = CliArgs::from_args();
    logger::configure_logger(args.verbose);

    run(args.input_file_path).await
}
