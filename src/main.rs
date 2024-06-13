use std::{
    fs::{create_dir_all, File},
    io::{self, Result},
    path::{Path, PathBuf},
};

use clap::Parser;
mod config;
use config::_Config;

mod parse;
use crate::config::get_config_path;

/// A command-line journal logger, scheduler and task manager.
#[derive(Parser, Debug)]
struct Args {
    /// Path to config-file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_file = match args.config {
        Some(path) => Ok(path),
        None => get_config_path(),
    }
    .map_err(|e| {
        eprintln!("Error encountered: {e:?}");
        io::ErrorKind::NotFound
    })?;

    if !config_file.exists() {
        create_dir_all(
            config_file
                .parent()
                .unwrap_or(Path::new("/home/nobody_has_this_as_home_dir")),
        )
        .map_err(|e| {
            eprintln!("Failed to create parent directories");
            eprintln!("{e:?}");
            io::ErrorKind::NotFound
        })?;

        {
            let file = File::create(&config_file).map_err(|e| {
                eprintln!("Error occured: {e:?}");
                io::ErrorKind::NotFound
            })?;

            file.sync_all().map_err(|e| {
                eprintln!("Error occured: {e:?}");
                io::ErrorKind::NotFound
            })?;
        }

        config::write_default_config(&config_file).map_err(|e| {
            eprintln!("Error occured while writing the default config.");
            eprintln!("{e:?}");
            io::ErrorKind::InvalidInput
        })?;
    }

    let config = _Config::try_parse(&config_file).map_err(|e| {
        eprintln!("Error occured in reading config file!");
        eprintln!("{e:?}");
        io::ErrorKind::NotFound
    })?;
    todo!();
}
