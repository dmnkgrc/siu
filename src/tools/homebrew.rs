use std::process::{Command, Stdio};

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

        let output = Command::new("/bin/bash")
            .args(["-c", "\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""])
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to install Homebrew");

        if !output.status.success() {
            panic!(
                "{}",
                String::from_utf8(output.stderr).expect("got non UTF-8 data from stdout")
            )
        }

        Ok(())
    }
}

pub fn install_package(name: &str) -> Result<(), &'static str> {
    let mut error_msg = "Failed to install package ".to_owned();
    error_msg.push_str(name);
    println!("Will install {} with Homebrew", name);
    let output = Command::new("brew")
        .args(["install", name])
        .stdout(Stdio::inherit())
        .output()
        .expect(&error_msg);

    if output.status.success() {
        return Ok(());
    }
    panic!("{}", String::from_utf8(output.stderr).expect(&error_msg))
}
