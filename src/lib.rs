use serde::{Deserialize};
use serde_json::{self, Result, Value};

#[derive(Deserialize, Debug)]
pub struct Row {
    pub cells: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Table {
    pub rows: Vec<Row>,
}

impl Table {

    pub fn new(data: &str) -> Result<Table> {
        let table = serde_json::from_str(data);
        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_string() {
        let test_table = r#"
[
  ["0,0", "0,1", "0,2"],
  ["1,0", "1,1", "1,2"],
  ["2,0", "2,1", "2,2"]
]"#;
        // let table = Table::new(&test_table).unwrap();
        // assert_eq!(table.rows.len(), 3);
        let parsed: Value = serde_json::from_str(&test_table).unwrap();
        match parsed {
            Value::Array(value) => {
                assert_eq!(value.len(), 3);
            },
            _ => (),
        }
    }

    #[test]
    fn from_json_string_to_table() {
        let test_table = r#"
[
  ["0,1", "0,2", "0,3"],
  ["1,1", "1,2", "1,3"]
]
"#;
        let table: Table = Table::new(&test_table).unwrap();
        assert_eq!(table.rows.len(), 2);
    }
}

