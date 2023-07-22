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
        let shell = shell::get_current().expect("Failed to get current shell");
        if let Some(install) = self.install {
            if install && tool_step == 0 {
                let brew = Homebrew::Packages(String::from("rbenv"));
                brew.install(tool_step)?;
                let rbenv_shell_config = match shell {
                    shell::Shell::Bash => "eval \"$(rbenv init - bash)\"",
                    shell::Shell::Zsh => "eval \"$(rbenv init - zsh)\"",
                    shell::Shell::Fish => "status --is-interactive; and rbenv init - fish | source",
                };
                if !shell.config_contains_string(rbenv_shell_config) {
                    println!("Adding rbenv config to {} config file", shell.name());
                    if let Err(e) = shell.write_to_config(rbenv_shell_config) {
                        return Err(format!(
                            "Failed to write rbenv config to {} config: {}",
                            shell.name(),
                            e
                        ));
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
        }

        if !shell.has_command("rbenv") {
            return Err(
                "rbenv not found, make sure you completed the previous step before continuing"
                    .to_string(),
            );
        }

        self.print_command();
        let child = Command::new("rbenv")
            .args(["install", &self.ruby_version])
            .spawn()
            .unwrap();
        let output = child.wait_with_output().expect("Failed to run command");
        if !output.status.success() {
            let err_output = String::from_utf8(output.stderr).unwrap();
            // Rbenv handles aborting when a ruby version is already installed with
            // an exit code of 1, but it is not an error.
            if !err_output.is_empty() {
                return Err("Failed to install ruby version".to_string());
            }
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
