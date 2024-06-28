use std::{
    fs::{create_dir_all, File},
    io::{self, Result},
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
mod config;
use config::{get_config_path, UpperConfig};
use parse::parse_sequence;
use tables::{print_comments, print_todos};

mod parse;
mod structs;
mod tables;

/// A command-line journal logger, scheduler and task manager.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to config-file
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[arg(short, long)]
    notes: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
    /// Start date
    #[arg(short, long)]
    start_date: Option<NaiveDate>,
    /// End date
    #[arg(short, long)]
    end_date: Option<NaiveDate>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows a list of incomplete todos
    Todo,
    /// Shows you a schedule of your day
    Schedule,
    /// Shows you the logs you record throughout your days
    Logs,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut config_file = match args.config {
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

        let file = File::create(&config_file).map_err(|e| {
            eprintln!("Error occured: {e:?}");
            io::ErrorKind::NotFound
        })?;

        file.sync_all().map_err(|e| {
            eprintln!("Error occured: {e:?}");
            io::ErrorKind::InvalidData
        })?;

        config::write_default_config(&config_file).map_err(|e| {
            eprintln!("Error occured while writing the default config.");
            eprintln!("{e:?}");
            io::ErrorKind::InvalidInput
        })?;
    }

    let config = UpperConfig::try_parse(&config_file).map_err(|e| {
        eprintln!("Error occured in reading config file!");
        eprintln!("{e:?}");
        io::ErrorKind::InvalidData
    })?;

    let start_date = args.start_date.unwrap_or(config.start_date);
    let end_date = args.end_date.unwrap_or(Local::now().date_naive());

    let schedule = parse_sequence(
        &start_date,
        &end_date,
        &mut PathBuf::from_str(&config.notes_folder).unwrap(),
    );
    print_todos(&schedule.tbd_todos, &config.todos);
    print_comments(&schedule.comments, &config.comments);

    Ok(())
}
