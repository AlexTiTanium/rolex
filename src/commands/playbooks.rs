use log::{error, info};
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

use crate::utils;

///
/// Command `hosts create`
///
pub fn run_playbook(host_group: &str, playbook_name: &str, playbook: &str) {
    match utils::is_installed("ansible-playbook") {
        Ok(_) => (), // Do nothing
        Err(_) => {
            error!("Can't find ansible-playbook, try run install command first");
            return; // Exit
        }
    }

    info!("Execute playbook {playbook_name} for {host_group}...");

    // Create temporary file
    let mut temp_file = NamedTempFile::new().expect("Unable to create temp file");
    temp_file
        .write_all(playbook.as_bytes())
        .expect("Unable to write to temp file");

    // Run Ansible
    let status = Command::new("ansible-playbook")
        .arg("-i")
        .arg(utils::get_host_config_path())
        .arg("--limit")
        .arg(host_group)
        .arg(temp_file.path())
        .status()
        .expect("failed to execute ansible-playbook");

    // Report execution  status
    if status.success() {
        info!("Playbook {playbook_name} executed successful for {host_group}");
    } else {
        error!("Failed to run ansible-playbook exited with {}", status);
    }
}
