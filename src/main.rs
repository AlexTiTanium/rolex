extern crate clap;
extern crate log;
extern crate pretty_env_logger;

mod commands;
mod utils;

use clap::{App, SubCommand};
use log::info;
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
        .subcommand(SubCommand::with_name("install").about("Installs ansible-playbook"))
        .subcommand(
            SubCommand::with_name("init").about("Initializes hosts.ini with your servers list"),
        )
        .subcommand(SubCommand::with_name("setup").about("Initial server setup command"))
        .subcommand(
            SubCommand::with_name("user").about("Create new user and return back user private key"),
        )
        .subcommand(SubCommand::with_name("reload").about("Reload service"))
        .get_matches();

    match matches.subcommand_name() {
        Some("install") => {
            commands::install_command();
        }
        Some("init") => {
            info!("Initializing hosts.ini...");
            // TODO: Actual logic
        }
        Some("setup") => {
            info!("Running setup...");
            // TODO: Actual logic
        }
        Some("user") => {
            info!("Managing user...");
            // TODO: Actual logic
        }
        Some("reload") => {
            info!("Reloading service...");
            // TODO: Actual logic
        }
        _ => {
            info!("Invalid command, run with --help for usage.");
        }
    }
}
