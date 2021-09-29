use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::path::PathBuf;
use std::sync::Mutex;
use structopt::StructOpt;

mod error;

use crate::error::AppError;

#[derive(Debug, Default)]
pub struct TypeAnalyzer {
    type_map: Mutex<HashMap<String, TypeData>>,
}

impl TypeAnalyzer {
    pub fn new() -> Self {
        Self {
            type_map: Mutex::new(HashMap::new()),
        }
    }

    pub async fn check_line(&mut self, line: String) -> Result<(), AppError> {
        let serde_value: serde_json::Map<String, Value> = serde_json::from_str(&line)?;
        println!("{:?}", serde_value);
        if let Some(type_val) = serde_value.get("type") {
            match type_val.as_str() {
                Some(name) => {
                    let size = Self::calculate_size(&serde_value);
                    let mut db = self.type_map.lock().expect("Lock poisoned!");
                    if let Some(val) = db.get_mut(name) {
                        val.byte_size += size;
                        val.count += 1;
                    } else {
                        db.insert(
                            name.to_string(),
                            TypeData {
                                count: 1,
                                byte_size: size,
                            },
                        );
                    }
                }
                None => {
                    return Err(AppError::FieldTypeIsNotString(line));
                }
            }
            Ok(())
        } else {
            return Err(AppError::NoFieldTypeInJson(line));
        }
    }

    pub fn calculate_size(json_object_map: &serde_json::Map<String, Value>) -> u128 {
        let mut byte_size = 0;
        for (_, val) in json_object_map {
            match val {
                Value::Null => continue,
                Value::Object(val) => byte_size += Self::calculate_size(&val),
                _ => byte_size += mem::size_of_val(&val) as u128,
            }
        }
        byte_size
    }

    pub fn print_output(&self) {
        use prettytable::{cell, row, Cell, Row, Table};
        let cloned_map = self.type_map.lock().expect("Lock poisoned!").clone();

        let mut table = Table::new();
        table.add_row(row!["Type Name", "Count", "Byte Size"]);
        for (key, val) in cloned_map {
            table.add_row(Row::new(vec![
                Cell::new(&key),
                Cell::new(&val.count.to_string()),
                Cell::new(&val.byte_size.to_string()),
            ]));
        }
        table.printstd();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeData {
    count: u128,
    byte_size: u128,
}

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
