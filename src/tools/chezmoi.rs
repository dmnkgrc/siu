use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::process::Command;

use super::{homebrew::Homebrew, types::Tool};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Chezmoi {
    repo: String,
    apply: Option<bool>,
}

impl Chezmoi {
    fn get_repo_url(&self) -> String {
        format!("git@github.com/{}.git", self.repo)
    }
}

impl Tool for Chezmoi {
    fn install(&self, tool_step: usize) -> Result<bool, String> {
        let brew = Homebrew::Packages(String::from("chezmoi"));
        brew.install(tool_step)?;
        self.print_command();
        let repo_url = self.get_repo_url();
        let mut args = vec!["init", &repo_url];
        if let Some(apply) = self.apply {
            if apply {
                args.push("--apply");
                args.swap(1, 2);
            }
        }
        let mut child = Command::new("chezmoi").args(args).spawn().unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install chezmoi".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }

        Ok(false)
    }

    fn print_command(&self) {
        let apply = match self.apply.unwrap_or(false) {
            false => "",
            true => "--apply ",
        };
        println!(
            "\n{}\n",
            format!("chezmoi init {}{}", apply, self.get_repo_url()).italic()
        );
    }
}
