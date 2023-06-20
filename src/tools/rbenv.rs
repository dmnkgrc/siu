use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::shell;

use super::{homebrew::Homebrew, types::Tool};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Rbenv {
    install: Option<bool>,
    ruby_version: String,
    global: Option<bool>,
}

impl Tool for Rbenv {
    fn install(&self, tool_step: usize) -> Result<bool, String> {
        if let Some(install) = self.install {
            if install && tool_step == 0 {
                let shell = shell::get_current().expect("Failed to get current shell");
                let brew = Homebrew::Packages(String::from("rbenv"));
                brew.install(tool_step)?;
                match shell {
                    shell::Shell::Bash => {
                        println!("Adding rbenv config to bash config file");
                        if let Err(e) = shell.write_to_config("eval \"$(rbenv init - bash)\"") {
                            return Err(format!(
                                "Failed to write rbenv config to bash config: {}",
                                e
                            ));
                        }
                    }
                    shell::Shell::Zsh => {
                        println!("Adding rbenv config to zsh config file");
                        if let Err(e) = shell.write_to_config("eval \"$(rbenv init - zsh)\"") {
                            return Err(format!(
                                "Failed to write rbenv config to zsh config: {}",
                                e
                            ));
                        }
                    }
                    shell::Shell::Fish => {
                        println!("Adding rbenv config to fish config file");
                        if let Err(e) = shell.write_to_config(
                            "status --is-interactive; and rbenv init - fish | source",
                        ) {
                            return Err(format!(
                                "Failed to write rbenv config to fish config: {}",
                                e
                            ));
                        }
                    }
                }
                println!(
                    "\n{}",
                    "Open a new shell and run this command again to complete installation"
                        .purple()
                        .bold()
                );
                return Ok(true);
            }
        }
        self.print_command();
        let mut child = Command::new("rbenv")
            .args(["install", &self.ruby_version])
            .spawn()
            .unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install ruby version".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }
        if let Some(global) = self.global {
            if global {
                println!(
                    "\n{}\n",
                    format!("rbenv global {}", self.ruby_version).italic()
                );
                let mut child = Command::new("rbenv")
                    .args(["global", &self.ruby_version])
                    .spawn()
                    .unwrap();
                match child.wait() {
                    Ok(status) => {
                        if !status.success() {
                            return Err("Failed to set global ruby version".to_string());
                        }
                    }
                    Err(e) => return Err(format!("Failed to run command: {}", e)),
                }
            }
        }

        Ok(false)
    }
    fn print_command(&self) {
        println!(
            "\n{}\n",
            format!("rbenv install {}", self.ruby_version).italic()
        );
    }
}
