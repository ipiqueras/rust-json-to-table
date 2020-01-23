use tempfile::NamedTempFile;
use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::{self, Write};

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("json_to_table")?;
    cmd.arg("--json").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read input file"));
    Ok(())
}

#[test]
fn file_fails_to_parse() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = NamedTempFile::new()?;
    writeln!(file, "Something that is not JSON")?;

    let mut cmd = Command::cargo_bin("json_to_table")?;
    cmd.arg("--json").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Parse error on user input"));
    Ok(())
}

#[test]
fn file_ok() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = NamedTempFile::new()?;
    writeln!(file, r#"
[
  ["0,0", "0,1", "0,2"],
  ["1,0", "1,1", "1,2"],
  ["2,0", "2,1", "2,2"]
]"#
    )?;
    let mut cmd = Command::cargo_bin("json_to_table")?;
    cmd.arg("--json").arg(file.path());
    cmd.assert().success()
        .stdout(predicate::str::contains(r#"+-----+-----+-----+
| 0,0 | 0,1 | 0,2 |
+-----+-----+-----+
| 1,0 | 1,1 | 1,2 |
+-----+-----+-----+
| 2,0 | 2,1 | 2,2 |
+-----+-----+-----+
"#));
    Ok(())
}