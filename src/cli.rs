use structopt::StructOpt;
use std::path::PathBuf;

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