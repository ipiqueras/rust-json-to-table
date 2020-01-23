//! # JSON 2 table
//!
//! Library interface for **json_to_table** cli.
use serde::{Deserialize};
use serde_json::{self};
extern crate thiserror;
use thiserror::Error;
#[macro_use] extern crate prettytable;
use prettytable::Table as pTable;
use prettytable::Row as pRow;
use prettytable::Cell;

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

fn create_table(data: &str) -> std::result::Result<pTable, ValidationError> {

    let table: Table = serde_json::from_str(data)?;
    let mut pretty_table = pTable::new();
    for row_item in &table.rows {
        let cells: Vec<Cell> = row_item.cells.iter()
            .map(|x| Cell::new(x) )
            .collect();
        pretty_table.add_row(pRow::new(cells));
    }
    Ok(pretty_table)
}

/// Main application interface. Gets a string slice and tries to parse it as
/// JSON data.
///
/// Creates the pretty Table and prints to stdout
pub fn print_table(data: &str) -> std::result::Result<(), ValidationError> {

    let table = create_table(data)?;
    // let format = format::FormatBuilder::new().column_separator('&').right_border('\\').padding(1, 1).build();
    // pretty_table.set_format(format);
    table.printstd();
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
        let result = print_table(&test_table);
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
        let result = create_table(&test_table);
        match result {
            Ok(value) => assert_eq!(value.len(), 3),
            Err(_) => panic!("Should have created a table"),
        }
    }
}
