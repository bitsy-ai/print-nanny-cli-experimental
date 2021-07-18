use assert_cmd::prelude::*; // cli initializations and assertions based around Command
use predicates::prelude::*; // used for writing assertion statements (AKA predicates)
use assert_cmd::Command;
use anyhow::{ Result, };

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
fn test_print_config() -> Result<()> {
    // let config = PrintNannySystemConfig::default();
    // let expected = print_config(&config);
    let mut cmd = Command::cargo_bin("printnanny")?;
    cmd.args(&["config", "show"]);

    cmd.assert()
        .success();
        // .stdout(expected);
    Ok(())
}

#[cfg(target_arch="x86_64")]
#[test]
fn test_camera_add_config_error() -> Result<()> {
    let mut cmd = Command::cargo_bin("printnanny")?;
    cmd.args(&["--config", "test", "camera", "add"]);

    // expect empty config to throw error prompting user to run `printnanny auth`
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("printnanny auth"));
    Ok(())
}