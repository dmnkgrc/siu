use std::process::Command;

pub trait Tool {
    fn install(&self) -> Result<(), &'static str>;
}

pub fn is_already_installed(name: &str) -> Result<bool, &'static str> {
    let output = Command::new("which")
        .arg(name)
        .output()
        .expect("Failed to check if tool is installed");

    match String::from_utf8(output.stdout) {
        Ok(val) if output.status.success() && !val.trim().is_empty() => Ok(true),
        Ok(_) => Ok(false),
        Err(_) => panic!("got non UTF-8 data from stdout"),
    }
}
