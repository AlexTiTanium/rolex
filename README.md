# Role Executor CLI Tool

## Description

A CLI tool for managing server tasks. This tool integrates with Ansible and provides a dynamic set of commands for various host groups.

## Prerequisites

### Install Rust and Cargo

Before you can use this CLI tool, you need to install Rust and Cargo on your system.
To do so, you can run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your shell or run source $HOME/.cargo/env.

### Install the CLI tool via Cargo

You can install the Rolex CLI tool using Cargo with the following command:

```bash
cargo install rolex
```

## Usage

### Managing Hosts

```bash
rolex hosts create  # create a new hosts.ini file
rolex hosts edit    # open the hosts.ini file in the default editor
```

### Managing Host Groups

```bash
rolex {server_group} install caddy  # install Caddy web server on the selected host group
rolex {server_group} user add {username}  # add a new user to the server
```

## License

MIT
