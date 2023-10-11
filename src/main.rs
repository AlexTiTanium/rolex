extern crate clap;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Role Executor CLI tool")
        .version("0.1")
        .author("Alex Kucherenko")
        .about("RoleX CLI for managing various tasks")
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
            println!("Installing ansible-playbook...");
            // TODO: Actual logic
        }
        Some("init") => {
            println!("Initializing hosts.ini...");
            // TODO: Actual logic
        }
        Some("setup") => {
            println!("Running setup...");
            // TODO: Actual logic
        }
        Some("user") => {
            println!("Managing user...");
            // TODO: Actual logic
        }
        Some("reload") => {
            println!("Reloading service...");
            // TODO: Actual logic
        }
        _ => {
            println!("Invalid command, run with --help for usage.");
        }
    }
}
