use std::process::Command;

use super::{
    homebrew,
    tool::{is_already_installed, Tool},
};

pub struct Node {}

impl Tool for Node {
    fn install(&self) -> Result<(), &'static str> {
        let is_installed =
            is_already_installed("node").expect("Failed to check if tool is installed");

        if is_installed {
            println!("Node already installed! Skipping...\n\n");
            return Ok(());
        }

        let is_brew_installed =
            is_already_installed("brew").expect("Failed to check if brew is installed");

        let brew = homebrew::Homebrew {};

        if !is_brew_installed {
            println!("Homebrew is not installed! Installing it first...\n");
            brew.install().expect("Failed to install Homebrew");
        }

        homebrew::install_package("fnm")?;

        let env_output = Command::new("fnm")
            .arg("env")
            .output()
            .expect("Failed to get fnm env variables");

        if !env_output.status.success() {
            panic!(
                "{}",
                String::from_utf8(env_output.stderr).expect("got non UTF-8 data from stdout")
            )
        }

        // let env_variables =
        //     String::from_utf8(env_output.stdout).expect("got non UTF-8 data from stdout");

        Ok(())
    }
}
