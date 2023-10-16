extern crate clap;
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use clap::{App, Arg, SubCommand};
use log::{error, info, warn};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::Write,
};

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
        let hosts_commands = SubCommand::with_name(host_group)
            .about("Host group actions")
            .subcommands(vec![
                get_hosts_install_commands(),
                get_hosts_user_commands(),
            ]);

        app = app.subcommand(hosts_commands);
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
            handle_host_commands(dynamic_command, sub_matches, &available_hosts);
        }
        _ => {
            error!("Invalid command, run with --help for usage.");
        }
    }
}

fn get_hosts_install_commands() -> App<'static, 'static> {
    SubCommand::with_name("install")
        .about("Install application on selected host group")
        .subcommand(SubCommand::with_name("caddy").about("Install Caddy web server"))
}

fn get_hosts_user_commands() -> App<'static, 'static> {
    SubCommand::with_name("user")
        .about("Manage users on server")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add new user on server")
                .arg_from_usage("<user> 'The username to add'"),
        )
}

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
        ("install", Some(install_matches)) => handle_host_install_commands(install_matches),
        ("user", Some(user_matches)) => handle_host_user_commands(user_matches),
        _ => {
            error!("Invalid host command, run with --help to get available commands");
        }
    }
}

fn handle_host_install_commands(app_matches: &clap::ArgMatches) {
    match app_matches.subcommand_name() {
        Some("caddy") => {
            info!("Install caddy");
        }
        _ => {
            error!("Invalid install subcommand");
        }
    }
}

fn handle_host_user_commands(user_matches: &clap::ArgMatches) {
    match user_matches.subcommand() {
        ("add", Some(add_matches)) => {
            let username = add_matches.value_of("user").unwrap();
            info!("add user: {}", username);
        }
        _ => {
            error!("Invalid user subcommand");
        }
    }
}
