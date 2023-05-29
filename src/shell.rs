use std::{
    env,
    path::{Path, PathBuf},
};

pub enum Shell {
    Bash,
    Zsh,
    Fish,
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

pub fn get_shell_config_path(shell: &Shell) -> PathBuf {
    let home = env::var("HOME").unwrap();
    PathBuf::from(&match shell {
        Shell::Bash => format!("{}/.bashrc", home),
        Shell::Zsh => format!("{}/.zshrc", home),
        Shell::Fish => format!("{}/.config/fish/config.fish", home),
    })
}

pub fn shell_config_exists(shell: &Shell) -> bool {
    return Path::new(&get_shell_config_path(shell)).exists();
}
