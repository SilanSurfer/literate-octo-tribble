use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

mod analyzer;
mod error;

use crate::analyzer::TypeAnalyzer;
use crate::error::AppError;

#[derive(Debug, StructOpt)]
#[structopt(name = "Literate Octo Tribble", about = "CLI to analyse logs")]
pub struct CliArgs {
    /// The path to the file to read
    pub input_file_path: PathBuf,
    /// Path of the output file. By default output is printed into stdout.
    #[structopt(short = "o", long)]
    pub output_file_path: Option<PathBuf>,
    /// Verbosity level
    /// -v for debug,
    /// -vv for trace
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbose: u8,
}

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
