use std::process::Command;

use owo_colors::OwoColorize;

pub trait Tool {
    fn install(&self) -> Result<(), String>;
    fn print_command(&self);
}

pub struct Homebrew<'a> {
    packages: Vec<&'a str>,
}

impl<'a> Tool for Homebrew<'a> {
    fn install(&self) -> Result<(), String> {
        self.print_command();
        let args = [Vec::from(["install"]), self.packages.to_owned()].concat();
        let mut child = Command::new("brew").args(args).spawn().unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install brew packages".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }

        Ok(())
    }
    fn print_command(&self) {
        println!(
            "\n{}\n",
            format!("brew install {}", self.packages.join(" ")).italic()
        );
    }
}

pub struct Pnpm<'a> {
    packages: Vec<&'a str>,
}

impl<'a> Tool for Pnpm<'a> {
    fn install(&self) -> Result<(), String> {
        let args = [Vec::from(["install", "--global"]), self.packages.to_owned()].concat();
        let mut child = Command::new("pnpm").args(args).spawn().unwrap();
        match child.wait() {
            Ok(status) => {
                if !status.success() {
                    return Err("Failed to install pnpm global packages".to_string());
                }
            }
            Err(e) => return Err(format!("Failed to run command: {}", e)),
        }

        Ok(())
    }
    fn print_command(&self) {
        println!("pnpm install --global {}", self.packages.join(" "));
    }
}

pub struct Yarn<'a> {
    packages: Vec<&'a str>,
}

impl<'a> Tool for Yarn<'a> {
    fn install(&self) -> Result<(), String> {
        let args = [Vec::from(["global", "add"]), self.packages.to_owned()].concat();
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
        println!("yarn global add {}", self.packages.join(" "));
    }
}

pub trait Tools {
    fn brew(&self) -> Homebrew;
    fn pnpm(&self) -> Pnpm;
    fn yarn(&self) -> Yarn;
}

impl Tools for String {
    fn brew(&self) -> Homebrew {
        Homebrew {
            packages: self.split(' ').collect::<Vec<&str>>(),
        }
    }

    fn pnpm(&self) -> Pnpm {
        Pnpm {
            packages: self.split(' ').collect::<Vec<&str>>(),
        }
    }

    fn yarn(&self) -> Yarn {
        Yarn {
            packages: self.split(' ').collect::<Vec<&str>>(),
        }
    }
}
