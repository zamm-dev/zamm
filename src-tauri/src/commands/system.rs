use serde::{Deserialize, Serialize};
use specta::specta;
use specta::Type;

use std::env;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum OS {
    Mac,
    Linux,
    Windows,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub enum Shell {
    Bash,
    Zsh,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
pub struct SystemInfo {
    zamm_version: String,
    os: Option<OS>,
    shell: Option<Shell>,
    shell_init_file: Option<String>,
}

fn get_zamm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_os() -> Option<OS> {
    #[cfg(target_os = "linux")]
    return Some(OS::Linux);
    #[cfg(target_os = "macos")]
    return Some(OS::Mac);
    #[cfg(target_os = "windows")]
    return Some(OS::Windows);
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows"
    )))]
    return None;
}

fn get_shell() -> Option<Shell> {
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

fn get_relative_profile_init_file() -> Option<String> {
    #[cfg(target_os = "linux")]
    return Some("~/.profile".to_string());
    #[cfg(target_os = "macos")]
    return Some("~/.bash_profile".to_string());
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    return None;
}

fn get_shell_init_file(shell: &Option<Shell>) -> Option<String> {
    let relative_file = match shell {
        Some(Shell::Bash) => Some("~/.bashrc".to_string()),
        Some(Shell::Zsh) => Some("~/.zshrc".to_string()),
        Some(Shell::PowerShell) => None,
        None => get_relative_profile_init_file(),
    };
    relative_file
        .as_ref()
        .map(|f| shellexpand::tilde(f).to_string())
}

#[tauri::command(async)]
#[specta]
pub fn get_system_info() -> SystemInfo {
    let shell = get_shell();
    let shell_init_file = get_shell_init_file(&shell);

    SystemInfo {
        zamm_version: get_zamm_version(),
        os: get_os(),
        shell,
        shell_init_file,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::{DirectReturn, SampleCallTestCase};
    use cfg_if::cfg_if;

    struct GetSystemInfoTestCase {
        pub system_info: SystemInfo,
    }

    impl SampleCallTestCase<(), SystemInfo> for GetSystemInfoTestCase {
        const EXPECTED_API_CALL: &'static str = "get_system_info";
        const CALL_HAS_ARGS: bool = false;

        async fn make_request(&mut self, _args: &Option<()>) -> SystemInfo {
            self.system_info.clone()
        }

        fn serialize_result(&self, sample: &SampleCall, result: &SystemInfo) -> String {
            DirectReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&()>,
            result: &SystemInfo,
        ) {
            DirectReturn::check_result(self, sample, args, result).await
        }
    }

    impl DirectReturn<(), SystemInfo> for GetSystemInfoTestCase {}

    #[test]
    fn test_can_determine_zamm_version() {
        let zamm_version = get_zamm_version();
        println!("Determined Zamm version to be {}", zamm_version);
        assert!(!zamm_version.is_empty());
    }

    #[test]
    fn test_can_determine_os() {
        let os = get_os();
        println!("Determined OS to be {:?}", os);
        assert!(os.is_some());
    }

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

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_can_predict_shell_init_for_zsh() {
        let shell = Shell::Zsh;
        let shell_init_file = get_shell_init_file(&Some(shell));
        println!("Shell init file is {:?}", shell_init_file);
        assert!(shell_init_file.is_some());
        let file_path = shell_init_file.unwrap();
        assert!(file_path.starts_with('/'));
        assert!(file_path.ends_with(".zshrc"));
    }

    #[test]
    fn test_can_predict_profile_init() {
        let shell_init_file = get_shell_init_file(&None);
        println!("Shell init file is {:?}", shell_init_file);

        cfg_if! {
            if #[cfg(target_os = "windows")] {
                assert!(shell_init_file.is_none());
            } else {
                let file_path = shell_init_file.unwrap();
                assert!(file_path.starts_with('/'));
                assert!(file_path.ends_with(".profile"));
            }
        }
    }

    #[tokio::test]
    async fn test_get_linux_system_info() {
        let mut test_case = GetSystemInfoTestCase {
            system_info: SystemInfo {
                zamm_version: "0.0.0".to_string(),
                os: Some(OS::Linux),
                shell: Some(Shell::Zsh),
                shell_init_file: Some("/root/.zshrc".to_string()),
            },
        };
        test_case
            .check_sample_call("./api/sample-calls/get_system_info-linux.yaml")
            .await;
    }
}
