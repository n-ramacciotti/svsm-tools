// SPDX-License-Identifier: MIT
//
// Copyright (C) 2026 Nicola Ramacciotti
//
// Author: Nicola Ramacciotti <niko.ramak@gmail.com>

use clap::{Parser, Subcommand};
use std::{error::Error, process};
use svsm_tools::OcpHandler;
use svsm_tools::details::OcpSourceType;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all objects with their sources
    List,
    /// Read the entire content of a source based on its name
    Read {
        /// The name of the source to read from
        #[arg(short, long)]
        name: String,
    },
    /// Write a source based on its name
    Write {
        /// The name of the source to write to
        #[arg(short, long)]
        name: String,
        /// The data to write
        #[arg(short, long)]
        data: String,
    },
}

// todo: for the moment it prints only to stdout,
// but it should be able to write to a file as well

fn run(command: Commands) -> Result<(), Box<dyn Error>> {
    let ocp = OcpHandler::new();

    match command {
        Commands::List => {
            let objects = ocp.list_all_objects()?;
            for obj in objects {
                println!("object: {}", obj.name()?);
                let sources = ocp.list_all_object_sources(obj.name()?)?;
                for src in sources {
                    println!("\t- {}", src.name()?);
                    println!(
                        "\t\t- Type: {:?}\n\t\t- Writable: {}",
                        src.kind(),
                        src.writable()
                    );
                }
            }
        }
        Commands::Read { name } => {
            let bytes = ocp.read_entire_source(&name)?;
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", content);
            //TODO: source kind
        }
        Commands::Write { name, data } => {
            let bytes_to_write = data.as_bytes();
            let bytes_written =
                ocp.write_entire_source(&name,bytes_to_write)?;
            println!("Wrote {} bytes to {}.", bytes_written, name);
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli.command) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
