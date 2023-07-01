use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub fn name(&self) -> &str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
        }
    }

    pub fn get_config_path_str(&self) -> String {
        let home = env::var("HOME").unwrap();
        match self {
            Shell::Bash => format!("{}/.bash_profile", home),
            Shell::Zsh => format!("{}/.zshrc", home),
            Shell::Fish => format!("{}/.config/fish/config.fish", home),
        }
    }

    pub fn get_config_path(&self) -> PathBuf {
        let home = env::var("HOME").unwrap();
        PathBuf::from(&match self {
            Shell::Bash => format!("{}/.bash_profile", home),
            Shell::Zsh => format!("{}/.zshrc", home),
            Shell::Fish => format!("{}/.config/fish/config.fish", home),
        })
    }

    pub fn config_exists(&self) -> bool {
        return Path::new(&self.get_config_path()).exists();
    }

    pub fn config_contains_string(&self, s: &str) -> bool {
        fs::read_to_string(self.get_config_path_str())
            .unwrap()
            .contains(s)
    }

    pub fn write_to_config(&self, s: &str) -> Result<(), String> {
        let path = &self.get_config_path();
        let file = fs::read(path).expect("Failed to read shell config file");

        let mut contents = String::from_utf8_lossy(&file).to_string();
        if !self.config_contains_string(s) {
            contents.push_str(s);
            fs::write(path, contents.as_bytes()).expect("Failed to write to shell config file");
        }
        Ok(())
    }
}

pub fn get_current() -> Result<Shell, &'static str> {
    let env_var_key = "SHELL";
    match env::var(env_var_key) {
        Ok(val) => {
            if val.contains("fish") {
                return Ok(Shell::Fish);
            } else if val.contains("zsh") {
                return Ok(Shell::Zsh);
            }
            Ok(Shell::Bash)
        }
        Err(e) => panic!("couldn't interpret {env_var_key}: {e}"),
    }
}
