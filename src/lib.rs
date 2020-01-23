use serde::{Deserialize};
use serde_json::{self, Value};
extern crate thiserror;
use thiserror::Error;

/// A type to represent the output of validate_input
type ValidationResult = std::result::Result<u32, ValidationError>;

#[derive(Error, Debug)]
/// Custom error to represent all possible errors that might arise parsing user input
pub enum ValidationError {
    #[error("Parse error on user input")]
    Parse(#[from] serde_json::Error),
    #[error("JSON input does not look like a table")]
    Invalid
}

pub fn create_from_string(data: &str) -> ValidationResult {

    let parsed: Value = serde_json::from_str(&test_table)?;
    match parsed {
        Value::Array(value) => {
            assert_eq!(value.len(), 3);
        },
        _ => {
            return Err(ValidationError::Invalid)
        },
    }
    Ok((1))
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
        let parsed: Value = serde_json::from_str(&test_table).unwrap();
        match parsed {
            Value::Array(value) => {
                assert_eq!(value.len(), 3);
            },
            _ => (),
        }
    }

    #[test]
    fn json_parse_error() {
        let test_table = r#"
-> Not a valid json
  ["0,0", "0,1", "0,2"],
  ["1,0", "1,1", "1,2"],
  ["2,0", "2,1", "2,2"]
]"#;

    }
}

