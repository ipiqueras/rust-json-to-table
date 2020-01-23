use tempfile::NamedTempFile;
use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::{self, Write};

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("jira")?;
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

    let mut cmd = Command::cargo_bin("jira")?;
    cmd.arg("--json").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Parse error on user input"));
    Ok(())
}
