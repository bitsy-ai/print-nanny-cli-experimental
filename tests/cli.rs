use assert_cmd::prelude::*; // cli initializations and assertions based around Command
use predicates::prelude::*; // used for writing assertion statements (AKA predicates)
use std::process::Command; // runner

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("printnanny")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}