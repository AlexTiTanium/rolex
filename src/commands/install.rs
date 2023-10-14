extern crate log;
use crate::utils::{is_installed, is_wsl};
use log::{error, info, warn};
use std::env;
use std::process::Command;

///
/// Main command for installing ansible-playbook
///
pub fn install_command() {
    info!("Installing ansible...");

    match is_installed("ansible") {
        Ok(true) => warn!("Ansible is already installed."),
        Ok(false) => install_ansible().unwrap(),
        Err(e) => error!("Error: {}", e),
    }
}

///
/// Install ansible-playbook
///
fn install_ansible() -> std::io::Result<()> {
    let os = if is_wsl() { "linux" } else { env::consts::OS };

    match os {
        "linux" => {
            let managers = ["apt", "yum", "dnf", "zypper", "pacman"];
            for manager in managers.iter() {
                let check_cmd = format!("command -v {}", manager);
                let status = Command::new("sh").arg("-c").arg(&check_cmd).status()?;
                if status.success() {
                    let install_command = format!(
                        "sudo {} update && sudo {} install -y ansible",
                        manager, manager
                    );
                    let install_status = Command::new("sh")
                        .arg("-c")
                        .arg(&install_command)
                        .status()?;
                    if install_status.success() {
                        info!("Ansible installed successfully via {}.", manager);
                        return Ok(());
                    }
                }
            }
            info!("No supported package manager found. Please install Ansible manually: https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html");
        }
        "macos" => {
            let install_command = "brew install ansible";
            let status = Command::new("sh").arg("-c").arg(install_command).status()?;
            if status.success() {
                info!("Ansible installed successfully.");
            } else {
                warn!("Can't install over brew. Please install Ansible manually: https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html");
            }
        }
        "windows" => {
            let install_command = "choco install ansible";
            let status = Command::new("cmd.exe")
                .arg("/C")
                .arg(install_command)
                .status()?;
            if status.success() {
                info!("Ansible installed successfully.");
            } else {
                warn!("Can't install over choco. Please install Ansible manually: https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html");
            }
        }
        _ => warn!("Unsupported OS. Please install Ansible manually: https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html"),
    }
    Ok(())
}
