use super::tool::{is_already_installed, Tool};

pub struct Homebrew {}

impl Tool for Homebrew {
    fn install(&self) -> Result<(), &'static str> {
        let is_installed =
            is_already_installed("brew").expect("Failed to check if tool is installed");

        if is_installed {
            println!("Homebrew already installed! Skipping...\n\n");
            return Ok(());
        }

        Ok(())
    }
}
