#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_no_args() {
        let mut cmd =
            Command::cargo_bin("solitaire_cypher").expect("crate binary should be where expected");
        cmd.assert().failure().stderr(predicate::str::contains(
            "required arguments were not provided",
        ));
    }

    #[test]
    fn test_garbage_args() {
        let mut cmd =
            Command::cargo_bin("solitaire_cypher").expect("crate binary should be where expected");
        cmd.arg("--moosepoop");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("unexpected argument"));
    }

    #[test]
    fn test_illegal_arg_combo() {
        let mut cmd =
            Command::cargo_bin("solitaire_cypher").expect("crate binary should be where expected");
        cmd.arg("--encrypt")
            .arg("--decrypt")
            .arg("--passphrase moosepoop")
            .write_stdin("SOMETEXT");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("unexpected argument"));
    }

    #[test]
    fn test_unparsable_passphrase() {
        let mut cmd =
            Command::cargo_bin("solitaire_cypher").expect("crate binary should be where expected");
        cmd.arg("--passphrase")
            .arg("cryp%^&omicon")
            .arg("--encrypt")
            .write_stdin("SOLITAIRE");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("string contains non-letter"));
    }

    #[test]
    fn test_encrypt_happypath() {
        let mut cmd =
            Command::cargo_bin("solitaire_cypher").expect("crate binary should be where expected");
        cmd.arg("--passphrase")
            .arg("cryptonomicon")
            .arg("--encrypt")
            .write_stdin("SOLITAIRE");
        cmd.assert().success().stdout("KIRAK SFJAN\n");
    }

    #[test]
    fn test_decrypt_happypath() {
        let mut cmd =
            Command::cargo_bin("solitaire_cypher").expect("crate binary should be where expected");
        cmd.arg("--decrypt")
            .arg("--passphrase")
            .arg("cryptonomicon")
            .write_stdin("KIRAK SFJAN");
        cmd.assert().success().stdout("SOLITAIREX\n");
    }
}
