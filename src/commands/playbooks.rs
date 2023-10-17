use log::{error, info};
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

use crate::utils;

///
/// Command `hosts create`
///
pub fn run_playbook(host_group: &str, playbook: &str, extra_vars: Option<HashMap<&str, &str>>) {
    match utils::is_installed("ansible-playbook") {
        Ok(_) => (), // Do nothing
        Err(_) => {
            error!("Can't find ansible-playbook, try run install command first");
            return; // Exit
        }
    }

    info!("Execute playbook for {host_group}...");

    // Create temporary file
    let mut temp_file = NamedTempFile::new().expect("Unable to create temp file");
    temp_file
        .write_all(playbook.as_bytes())
        .expect("Unable to write to temp file");

    // Create CMD
    let mut cmd = Command::new("ansible-playbook");

    // Add arguments
    cmd.arg("-i")
        .arg(utils::get_host_config_path())
        .arg("--limit")
        .arg(host_group)
        .arg(temp_file.path());

    // Add extra vars, if any
    if let Some(vars) = extra_vars {
        let extra_vars_str: Vec<String> =
            vars.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
        let extra_vars_arg = extra_vars_str.join(",");
        if !extra_vars_arg.is_empty() {
            cmd.arg("-e").arg(extra_vars_arg);
        }
    }

    // Execute command
    let status = cmd.status().expect("failed to execute ansible-playbook");

    // Report execution  status
    if status.success() {
        info!("Playbook executed successful for {host_group}");
    } else {
        error!("Failed to run ansible-playbook exited with {}", status);
    }
}
