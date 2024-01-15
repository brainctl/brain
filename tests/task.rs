use std::fs;
use std::io::Write;
use std::process::{Child, Stdio};
use predicates::str::*;

use assert_cmd::Command;

const BINARY_NAME: &str = "brain";

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_output_matches_file() {
        // Read the expected output from the file
        let expected_output = fs::read_to_string("expected_output.txt")
            .expect("failed to read expected output file");

        // Run the binary and capture the output
        let mut cmd = Command::cargo_bin("your_binary_name")
            .expect("binary not found");

        let output = cmd.write_stdin("your command input here")
            .unwrap()
            .output()
            .expect("failed to execute command");

        let output_str = String::from_utf8(output.stdout)
            .expect("failed to parse output as UTF-8");

        // Assert that the actual output matches the expected output
        assert_eq!(output_str, expected_output);
    }

    fn assert_stdout_equals(mut child: Child, path: &str) {
        // Read the output from stdout of the child process
        let mut output = String::new();
        child.stdout.as_mut()
            .expect("failed to open stdout")
            .read_to_string(&mut output)
            .expect("failed to read stdout");

        // Read the expected output from file
        let expected_output = fs::read_to_string(path)
            .expect("failed to read expected output file");

        // Assert that the actual output matches the expected output
        assert_eq!(output, expected_output);
    }
}
