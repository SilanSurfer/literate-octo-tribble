use crate::error::AppError;
use serde_json::Value;
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeData {
    count: u128,
    byte_size: u128,
}

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
        log::trace!("Checking object: {:?}", serde_value);
        if let Some(type_val) = serde_value.get("type") {
            match type_val.as_str() {
                Some(name) => {
                    let size = Self::calculate_size(&serde_value);
                    let mut db = self.type_map.lock().expect("Lock poisoned!");
                    if let Some(val) = db.get_mut(name) {
                        val.byte_size += size;
                        val.count += 1;
                        log::debug!("Updating type {} with byte size: {}", name, size);
                    } else {
                        db.insert(
                            name.to_string(),
                            TypeData {
                                count: 1,
                                byte_size: size,
                            },
                        );
                        log::debug!("Inserting type {} with byte size: {}", name, size);
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
        log::trace!("Calculating size for {:?}", json_object_map);
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
        table.print_tty(true);
    }
}
