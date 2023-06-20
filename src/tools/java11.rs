use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::shell;

use super::{homebrew::Homebrew, types::Tool};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Java11 {}

impl Tool for Java11 {
    fn install(&self, tool_step: usize) -> Result<bool, String> {
        let shell = shell::get_current().expect("Failed to get current shell");
        let brew = Homebrew::Packages(String::from("--cask homebrew/cask-versions/zulu11"));
        brew.install(tool_step)?;
        println!(
            "\nAdding JAVA_HOME environment variable to {} config file\n",
            shell.name()
        );
        match shell {
            shell::Shell::Fish => {
                if let Err(e) = shell.write_to_config(
                    "set -x JAVA_HOME /Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home",
                ) {
                    return Err(format!(
                        "Failed to write rbenv config to fish config: {}",
                        e
                    ));
                }
            }
            _ => {
                if let Err(e) = shell.write_to_config("export JAVA_HOME=\"/Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home\"") {
                    return Err(format!("Failed to write rbenv config to {} config: {}", shell.name(), e));
                }
            }
        }
        println!(
            "{}\n",
            "Make sure to open a new shell before using the JDK"
                .yellow()
                .italic()
        );

        Ok(false)
    }
    fn print_command(&self) {}
}
