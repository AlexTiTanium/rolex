use dirs::home_dir;
use log::{error, info, warn};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

///
/// Command `hosts create`
///
pub fn hosts_create_command() {
    info!("Creating host.ini config file...");

    match create_hosts_config() {
        Ok(()) => (),
        Err(e) => error!("Create hosts.ini error: {}", e),
    }
}

///
/// Command `hosts edit`
///
pub fn hosts_edit_command() {
    info!("Open hosts.ini file...");

    match open_hosts_in_default_editor() {
        Ok(()) => (),
        Err(e) => error!("Open hosts.ini error: {}", e),
    }
}

///
/// Open hosts.init in default user editor
///
fn open_hosts_in_default_editor() -> std::io::Result<()> {
    let hosts_file_path = "~/.rolex/hosts.ini";

    let os = env::consts::OS;
    match os {
        "linux" | "macos" => {
            let path = "~/.rolex/hosts.ini";
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            Command::new(editor).arg(path).status()?;
        }
        "windows" => {
            Command::new("cmd")
                .arg("/C")
                .arg("start")
                .arg(hosts_file_path)
                .status()?;
        }
        _ => println!("Unsupported OS"),
    }

    Ok(())
}

///
/// Create host file at .rolex/hosts.ini
/// In this file we will store all user servers
///
fn create_hosts_config() -> std::io::Result<()> {
    // Get path to home directory
    let home = home_dir().expect("Home directory not found");
    let app_dir = home.join(".rolex");

    // Create folder if not exist
    if !app_dir.exists() {
        std::fs::create_dir(&app_dir)?;
    }

    // Copy hosts to app folder
    let hosts_file_path = app_dir.join("hosts.ini");
    if !hosts_file_path.exists() {
        // Copy file content
        let hosts_content = include_str!("../config/hosts.ini");
        let mut file = File::create(&hosts_file_path)?;
        file.write_all(hosts_content.as_bytes())?;
        info!(
            "Hosts file successful created, now you can add your servers to: {}. Or use 'hosts edit' command to open host in your default editor",
            &hosts_file_path.display()
        );
    } else {
        warn!("Hosts file already exists, do nothing. Use 'hosts edit' command to open host.ini in your default editor");
    }

    Ok(())
}
