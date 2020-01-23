use serde::{Deserialize};
use serde_json::{self};
extern crate thiserror;
use thiserror::Error;
#[macro_use] extern crate prettytable;
use prettytable::Table as pTable;
use prettytable::Row as pRow;
use prettytable::Cell;

/// A type to represent the output of validate_input
type ValidationResult = std::result::Result<(), ValidationError>;

#[derive(Error, Debug)]
/// Custom error to represent all possible errors that might arise parsing user input
pub enum ValidationError {
    #[error("Parse error on user input")]
    Parse(#[from] serde_json::Error),
    #[error("JSON input does not look like a table")]
    Invalid
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Row { pub cells: Vec<String>, }

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Table { pub rows: Vec<Row>, }

impl Table {

    fn new_from_json(data: &str) -> std::result::Result<Table, ValidationError> {
        let table = serde_json::from_str(data)?;
        Ok(table)
    }
}

///
pub fn run(data: &str) -> ValidationResult {

    let table = Table::new_from_json(data)?;
    let mut pretty_table = pTable::new();
    for row_item in &table.rows {
        let mut cells: Vec<Cell> = row_item.cells.iter()
            .map(|x| Cell::new(x) )
            .collect();
        pretty_table.add_row(pRow::new(cells));
    }
    pretty_table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_parse_error() {
        let test_table = r#"
-> Not a valid json
  ["0,0", "0,1", "0,2"],
  ["1,0", "1,1", "1,2"],
  ["2,0", "2,1", "2,2"]
]"#;
        let result = run(&test_table);
        assert!(result.is_err());
    }

    #[test]
    fn json_parse_ok() {
        let test_table = r#"
[
  ["0,0", "0,1", "0,2"],
  ["1,0", "1,1", "1,2"],
  ["2,0", "2,1", "2,2"]
]"#;
        let result = run(&test_table);
        assert!(result.is_ok());
    }
}
