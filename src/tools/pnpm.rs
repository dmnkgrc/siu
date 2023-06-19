use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::process::Command;

use super::types::Tool;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Pnpm {
    Packages(String),
}

impl Pnpm {
    fn get_packages(&self) -> Vec<&str> {
        let Pnpm::Packages(s) = self;
        s.split(' ').collect::<Vec<&str>>()
    }
}

impl Tool for Pnpm {
    fn install(&self, _: usize) -> Result<bool, String> {
        let args = [
            Vec::from(["install", "--global"]),
            self.get_packages().to_owned(),
        ]
        .concat();
        let mut child = Command::new("pnpm").args(args).spawn().unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install pnpm global packages".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }

        Ok(false)
    }
    fn print_command(&self) {
        let Pnpm::Packages(s) = self;
        println!("\n{}\n", format!("pnpm install --global {}", s).italic());
    }
}
