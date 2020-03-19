use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn int_does_file_exist() -> Result<(), Box <dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar")
        .arg("test/file/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
 
#[test]
fn int_find_match_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Here is line one \nAnd here is line two\n we also can play with 유니 코드")?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("line")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Here is line one \nAnd here is line two"));
    Ok(())
}

#[test]
fn int_match_unicode_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Here is line one \nAnd here is line two\n we also can play with 유니 코드")?;
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("유니")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("we also can play with 유니 코드"));
    Ok(())
}