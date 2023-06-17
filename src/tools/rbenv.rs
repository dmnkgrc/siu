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
    fn install(&self) -> Result<(), String> {
        if let Some(install) = self.install {
            if install {
                let brew = Homebrew::Packages(String::from("rbenv"));
                brew.install()?;
                let shell = shell::get_current().expect("Failed to get current shell");
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
                let mut source_cmd = Command::new("ls")
                    .arg(shell.get_config_path_str())
                    .spawn()
                    .unwrap();
                match source_cmd.wait() {
                    Ok(status) => {
                        if !status.success() {
                            return Err("Failed to source shell config".to_string());
                        }
                    }
                    Err(e) => return Err(format!("Failed to run command: {}", e)),
                }
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

        Ok(())
    }
    fn print_command(&self) {
        println!(
            "\n{}\n",
            format!("rbenv install {}", self.ruby_version).italic()
        );
    }
}
