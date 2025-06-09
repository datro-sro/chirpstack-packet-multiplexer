use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn logging_output_unaffected_by_rust_log() {
    let mut cmd = Command::cargo_bin("eui_logger").unwrap();
    cmd.env("RUST_LOG", "warn");
    cmd.assert()
        .success()
        .stdout(contains("PREFIX"))
        .stdout(contains("EUI"));
}
