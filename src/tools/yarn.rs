use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::process::Command;

use super::types::Tool;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Yarn {
    Packages(String),
}

impl Yarn {
    fn get_packages(&self) -> Vec<&str> {
        let Yarn::Packages(s) = self;
        s.split(' ').collect::<Vec<&str>>()
    }
}

impl Tool for Yarn {
    fn install(&self) -> Result<(), String> {
        let args = [Vec::from(["global", "add"]), self.get_packages().to_owned()].concat();
        let mut child = Command::new("yarn").args(args).spawn().unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install yarn global packages".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }

        Ok(())
    }
    fn print_command(&self) {
        let Yarn::Packages(s) = self;
        println!("\n{}\n", format!("yarn global add {}", s).italic());
    }
}
