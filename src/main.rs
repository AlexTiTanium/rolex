extern crate clap;
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use clap::{App, SubCommand};
use log::{error, info, warn};
use std::{collections::HashSet, io::Write};

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log::LevelFilter::Info)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            writeln!(buf, "{}", level_style.value(record.args()),)
        })
        .init();

    let mut app = App::new("Role Executor CLI tool")
        .version("0.1")
        .author("Alex Kucherenko")
        .about("CLI tool for managing server tasks")
        .subcommand(SubCommand::with_name("install").about("Installs ansible dependency"))
        .subcommand(
            SubCommand::with_name("hosts")
                .about("Manage hosts.ini with your servers list")
                .subcommand(
                    SubCommand::with_name("create").about("Creates hosts.ini at ./rolex/hosts.ini"),
                )
                .subcommand(
                    SubCommand::with_name("edit").about("Opens hosts.ini in default editor"),
                ),
        );

    let available_hosts = match utils::get_available_hosts() {
        Ok(hosts) => hosts,
        Err(e) => {
            warn!("Can't read hosts.ini file: {}", e);
            warn!("Create host file with 'hosts create' command or add new host by: 'hosts edit'");
            HashSet::new()
        }
    };

    // Create dynamic hosts commands
    for host_group in &available_hosts {
        app = app.subcommand(
            SubCommand::with_name(host_group)
                .about("Host group actions")
                .subcommand(SubCommand::with_name("setup").about("Setup selected host group")),
        );
    }

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
        (dynamic_command, Some(sub_matches)) => {
            if available_hosts.contains(dynamic_command) {
                match sub_matches.subcommand() {
                    ("setup", Some(_)) => {
                        commands::run_playbook(
                            dynamic_command,
                            "setup",
                            include_str!("playbooks/setup.yml"),
                        );
                    }
                    _ => {
                        info!("Invalid host command, run with --help for to get available commands")
                    }
                }
            }
        }
        _ => {
            info!("Invalid command, run with --help for usage.");
        }
    }
}
