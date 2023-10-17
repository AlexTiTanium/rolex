#[macro_use]
extern crate maplit;

extern crate clap;
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use clap::{App, SubCommand};
use log::{error, warn};
use std::{collections::HashSet, io::Write};

/// Entry point for the Rolex CLI tool.
/// Initializes the logger, defines command-line arguments, and dispatches them to corresponding handlers.
fn main() {
    // Setup logger format
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log::LevelFilter::Info)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            writeln!(buf, "{}", level_style.value(record.args()))
        })
        .init();

    // CMD Config starts here
    let mut app = App::new("Role Executor CLI tool")
        .version("0.1")
        .author("Alex Kucherenko")
        .about("CLI tool for managing server tasks")
        .subcommand(SubCommand::with_name("install").about("Installs ansible dependency"))
        .subcommands([get_hosts_config_commands()]);

    // Get available host to add each to CLI
    let available_hosts = match utils::get_available_hosts() {
        Ok(hosts) => hosts,
        Err(e) => {
            warn!("Can't read hosts.ini file: {}", e);
            warn!("Create host file with 'hosts create' command or add new host by: 'hosts edit'");
            HashSet::new()
        }
    };

    // Create dynamic hosts commands for each host
    for host_group in &available_hosts {
        let hosts_commands = SubCommand::with_name(host_group)
            .about("Host group actions")
            .subcommands(vec![
                get_hosts_install_commands(),
                get_hosts_user_commands(),
            ]);

        app = app.subcommand(hosts_commands);
    }

    // commands handler
    match app.get_matches().subcommand() {
        ("install", _) => {
            commands::install_command();
            commands::hosts_create_command();
        }
        ("hosts", Some(hosts_matches)) => match hosts_matches.subcommand_name() {
            Some("create") => commands::hosts_create_command(),
            Some("edit") => commands::hosts_edit_command(),
            _ => error!("Invalid hosts subcommand"),
        },
        // Dynamic commands for hosts
        (dynamic_command, Some(sub_matches)) => {
            handle_host_commands(dynamic_command, sub_matches, &available_hosts);
        }
        _ => {
            error!("Invalid command, run with --help for usage.");
        }
    }
}

/// Get hosts.ini operations commands
///
/// Examples:
/// ```
/// $ rolex host create
/// $ rolex host edit
/// ```
fn get_hosts_config_commands() -> App<'static, 'static> {
    SubCommand::with_name("hosts")
        .about("Manage hosts.ini with your servers list")
        .subcommand(SubCommand::with_name("create").about("Creates hosts.ini at ./rolex/hosts.ini"))
        .subcommand(SubCommand::with_name("edit").about("Opens hosts.ini in default editor"))
}

/// Get host install {app_name} commands
///
/// # Examples
/// ```sh
/// $ rolex {server_group} install caddy
/// ````
///
fn get_hosts_install_commands() -> App<'static, 'static> {
    SubCommand::with_name("install")
        .about("Install application on selected host group")
        .subcommand(SubCommand::with_name("caddy").about("Install Caddy web server"))
}

/// Managing users on the servers commands
///
/// # Examples
/// ```sh
/// $ rolex {server_group} user add {user_name}
/// ````
fn get_hosts_user_commands() -> App<'static, 'static> {
    SubCommand::with_name("user")
        .about("Manage users on server")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add new user on server")
                .arg_from_usage("<user> The username to add"),
        )
}

/// Handles dynamic commands related to host groups.
///
/// # Example
/// ```sh
/// $ rolex {server_group} install {app_name}
/// $ rolex {server_group} user add {user_name}
/// ````
///
/// # Arguments
///
/// * `host` - The name of the host group to manage.
/// * `sub_matches` - The subcommand and its arguments as parsed by clap.
/// * `available_hosts` - A reference to the set of available host groups.
fn handle_host_commands(
    host: &str,
    sub_matches: &clap::ArgMatches,
    available_hosts: &HashSet<String>,
) {
    // If not host
    if !available_hosts.contains(host) {
        return;
    }

    match sub_matches.subcommand() {
        ("install", Some(install_matches)) => handle_host_install_commands(host, install_matches),
        ("user", Some(user_matches)) => handle_host_user_commands(host, user_matches),
        _ => {
            error!("Invalid host command, run with --help to get available commands");
        }
    }
}

/// Handles the `install` subcommands for the given host group
///
/// # Example
/// ```sh
/// $ rolex {server_group} install caddy
/// ````
///
/// # Arguments
///
/// * `host` - Host group where to execute install command
/// * `app_matches` - The subcommand and its arguments as parsed by clap.
fn handle_host_install_commands(host: &str, app_matches: &clap::ArgMatches) {
    match app_matches.subcommand_name() {
        Some("caddy") => {
            commands::run_playbook(host, include_str!("playbooks/install_caddy.yml"), None);
        }
        _ => {
            error!("Invalid install subcommand");
        }
    }
}

/// Handles the `user` subcommands for the given host group.
///
/// Example:
/// ```
/// $ rolex {server_group} user add
/// ```
///
/// Arguments:
///
/// * `host` - Host group where to execute install command
/// * `user_matches` - The subcommand and its arguments as parsed by clap.
fn handle_host_user_commands(host: &str, user_matches: &clap::ArgMatches) {
    match user_matches.subcommand() {
        ("add", Some(add_matches)) => {
            let username = match add_matches.value_of("user") {
                Some(username) => username,
                None => {
                    error!("No username, please add a username as the last argument");
                    return;
                }
            };

            commands::run_playbook(
                host,
                include_str!("playbooks/add_user.yml"),
                Some(hashmap! {
                    "user" => username,
                }),
            );
        }
        _ => {
            error!("Invalid user subcommand");
        }
    }
}
