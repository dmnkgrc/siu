use std::process::Command;

use super::tool::{is_already_installed, Tool};

pub struct Homebrew {}

impl Tool for Homebrew {
    fn install(&self) -> Result<(), &'static str> {
        let is_installed =
            is_already_installed("brew").expect("Failed to check if tool is installed");

        if is_installed {
            println!("Homebrew already installed! Skipping...\n");
            return Ok(());
        }

        Ok(())
    }
}

pub fn install_package(name: &str) -> Result<(), &'static str> {
    let mut error_msg = "Failed to install package ".to_owned();
    error_msg.push_str(name);
    let output = Command::new("brew")
        .args(["install", name])
        .output()
        .expect(&error_msg);

    if output.status.success() {
        return Ok(());
    }
    panic!("{}", String::from_utf8(output.stderr).expect(&error_msg))
}
