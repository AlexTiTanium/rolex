use configparser::ini::Ini;
use dirs;
use std::{collections::HashSet, env};

///
/// Read hosts.ini sections
///
pub fn get_available_hosts() -> Result<HashSet<String>, std::io::Error> {
    let home_dir = dirs::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Home directory not found",
    ))?;
    let path = home_dir.join(".rolex/hosts.ini");

    let mut config = Ini::new();
    config
        .load(path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut available_hosts: HashSet<String> = HashSet::new();

    for section in config.sections() {
        available_hosts.insert(section);
    }

    Ok(available_hosts)
}

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

///
/// Check if the system is WSL
///
pub fn is_wsl() -> bool {
    env::var("WSL_DISTRO_NAME").is_ok()
}
