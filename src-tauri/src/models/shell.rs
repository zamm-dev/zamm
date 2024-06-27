use serde::{Deserialize, Serialize};
use specta::Type;

use std::env;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Shell {
    Bash,
    Zsh,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
}

pub fn get_shell() -> Option<Shell> {
    if let Ok(shell) = env::var("SHELL") {
        if shell.ends_with("/zsh") {
            return Some(Shell::Zsh);
        }
        if shell.ends_with("/bash") {
            return Some(Shell::Bash);
        }
    }

    if env::var("ZSH_NAME").is_ok() {
        return Some(Shell::Zsh);
    }
    if env::var("BASH").is_ok() {
        return Some(Shell::Bash);
    }

    #[cfg(target_os = "windows")]
    return Some(Shell::PowerShell);
    #[cfg(not(target_os = "windows"))]
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_can_determine_shell() {
        let shell = get_shell();
        println!(
            "Determined shell to be {:?} from env var {:?}",
            shell,
            env::var("SHELL")
        );
        assert!(shell.is_some());
    }
}
