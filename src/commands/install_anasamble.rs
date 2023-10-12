use std::env;
use std::process::Command;

///
/// Main command for installing ansible-playbook
///
pub fn install_command() {
    println!("Installing ansible-playbook...");

    if is_ansible_installed() {
        println!("Ansible is already installed.");
    } else {
        install_ansible();
    }
}

///
///Install ansible-playbook
///
pub fn install_ansible() -> std::io::Result<()> {
    match is_ansible_installed() {
        Ok(_) => println!("Ansible is already installed."),
        Err(_) => {}
    }

    let os = env::consts::OS;
    match os {
        "linux" | "macos" => {
            let install_command = if os == "linux" {
                "sudo apt update && sudo apt install -y ansible"
            } else {
                "brew install ansible"
            };

            let status = Command::new("sh").arg("-c").arg(install_command).status()?;

            if status.success() {
                println!("Ansible installed successfully.");
            } else {
                println!("Failed to install Ansible.");
            }
        }
        "windows" => {
            let install_command = "choco install ansible";

            let status = Command::new("cmd.exe")
                .arg("/C")
                .arg(install_command)
                .status()?;

            if status.success() {
                println!("Ansible installed successfully.");
            } else {
                println!("Failed to install Ansible.");
            }
        }
        _ => {
            println!("Unsupported OS");
        }
    }
    Ok(())
}
