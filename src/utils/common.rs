use configparser::ini::Ini;
use dirs::home_dir;
use std::{collections::HashSet, env, path::PathBuf};

///
/// Get app directory
///
pub fn get_app_dir() -> PathBuf {
    let home = home_dir().expect("Home directory not found");
    return home.join(".rolex");
}

///
/// Get hosts config file path
///
pub fn get_host_config_path() -> PathBuf {
    get_app_dir().join("hosts.ini")
}

///
/// Read hosts.ini sections
///
pub fn get_available_hosts() -> Result<HashSet<String>, std::io::Error> {
    let path = get_host_config_path();

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
