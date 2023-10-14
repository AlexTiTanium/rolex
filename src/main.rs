extern crate clap;
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use clap::{App, SubCommand};
use log::{error, info};
use std::io::Write;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log::LevelFilter::Info)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            writeln!(buf, "{}", level_style.value(record.args()),)
        })
        .init();

    let matches = App::new("Role Executor CLI tool")
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
        )
        // .subcommand(SubCommand::with_name("setup").about("Initial server setup command"))
        // .subcommand(
        //     SubCommand::with_name("user").about("Create new user and return back user private key"),
        // )
        // .subcommand(SubCommand::with_name("reload").about("Reload service"))
        .get_matches();

    match matches.subcommand() {
        ("install", Some(_)) => {
            commands::install_command();
            commands::hosts_create_command();
        }
        ("hosts", Some(hosts_matches)) => match hosts_matches.subcommand_name() {
            Some("create") => commands::hosts_create_command(),
            Some("edit") => commands::hosts_edit_command(),
            _ => error!("Invalid hosts subcommand"),
        },
        _ => {
            info!("Invalid command, run with --help for usage.");
        }
    }
}
