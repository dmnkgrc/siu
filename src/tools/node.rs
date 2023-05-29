use super::{
    homebrew,
    tool::{is_already_installed, Tool},
};
use crate::shell;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    process::Command,
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

        println!("Finished installing Fnm");
        println!("Will add necessary env variables to shell config");

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

        let shell = shell::get_current().expect("Failed to get current shell");

        let shell_config_path = shell::get_shell_config_path(&shell);

        let mut config_file = OpenOptions::new()
            .append(true)
            .open(shell_config_path)
            .unwrap();

        if let Err(e) = config_file.write_all(&env_output.stdout) {
            eprintln!("Failed to write env variables to shell config: {}", e);
        }

        match shell {
            shell::Shell::Fish => {
                fs::write(
                    format!("{}/.config/fish/conf.d/fnm.fish", env::var("HOME").unwrap()),
                    "fnm env --use-on-cd | source",
                )
                .expect("Failed to write to fnm.fish");
            }
            _ => {
                if let Err(e) = write!(config_file, "eval \"$(fnm env --use-on-cd)\"") {
                    eprintln!("Failed to write script to automatically switch node versions to shell config: {}", e);
                }
            }
        }
        println!("Finished adding env variables to shell config");
        println!("Don't forget to source your shell config or start a new shell");

        Ok(())
    }
}
