use assert_cmd::prelude::*; // cli initializations and assertions based around Command
use predicates::prelude::*; // used for writing assertion statements (AKA predicates)
use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use printnanny::config::{ print_config, PrintNannySystemConfig };

// #[cfg(target_arch="x86_64")]
// #[test]
// fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("printnanny")?;

//     cmd.arg("foobar").arg("test/file/doesnt/exist");
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("No such file or directory"));
//     Ok(())
// }

#[cfg(target_arch="x86_64")]
#[test]
fn print_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = PrintNannySystemConfig::default();
    let expected = print_config(&config);
    let mut cmd = Command::cargo_bin("printnanny")?;
    cmd.arg("config");
    cmd.assert()
        .success();
        // .stdout(expected);
    Ok(())
}