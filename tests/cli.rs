use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs_hj")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("sample\nThis is good\ntry this\ngood sample")?;

    let mut cmd = Command::cargo_bin("grrs_hj")?;
    cmd.arg("sample").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sample\ngood sample"));

    Ok(())
}
