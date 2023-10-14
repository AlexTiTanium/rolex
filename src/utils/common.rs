use std::env;

///
/// Check if app installed on current system
///
pub fn is_installed(app: &str) -> Result<bool, std::io::Error> {
    let status = if cfg!(target_os = "windows") {
        std::process::Command::new("where.exe")
            .arg(app)
            .stdout(std::process::Stdio::null()) // Do not print command result
            .status()
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("which {}", app))
            .stdout(std::process::Stdio::null()) // Do not print command result
            .status()
    };

    match status {
        Ok(s) if s.success() => Ok(true),
        Ok(_) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Check if the system is WSL
pub fn is_wsl() -> bool {
    env::var("WSL_DISTRO_NAME").is_ok()
}
