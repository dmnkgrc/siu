use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::process::Command;

use super::types::Tool;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Homebrew {
    Packages(String),
}

impl Homebrew {
    fn get_packages(&self) -> Vec<&str> {
        let Homebrew::Packages(s) = self;
        s.split(' ').collect::<Vec<&str>>()
    }
}

impl Tool for Homebrew {
    fn install(&self, _: usize) -> Result<bool, String> {
        self.print_command();
        let args = [Vec::from(["install"]), self.get_packages().to_owned()].concat();
        let mut child = Command::new("brew").args(args).spawn().unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install brew packages".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }

        Ok(false)
    }

    fn print_command(&self) {
        let Homebrew::Packages(s) = self;
        println!("\n{}\n", format!("brew install {}", s).italic());
    }
}
