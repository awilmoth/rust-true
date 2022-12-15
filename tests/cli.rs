use assert_cmd::Command; 

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rust-true").unwrap(); 
    cmd.assert().success(); 
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("rust-true").unwrap();
    cmd.assert().success();
}
