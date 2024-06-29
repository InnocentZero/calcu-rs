use std::{
    fs::{create_dir_all, File},
    io::{self, Error, Result},
    path::{Path, PathBuf},
    str::FromStr,
};

use calcu_rs::parse::parse_sequence;
use calcu_rs::tables::{print_comments, print_todos};
use calcu_rs::{
    config::{get_config_path, write_default_config, UpperConfig},
    tables::print_schedule,
};

use chrono::{Days, Local, NaiveDate};
use clap::{Parser, Subcommand};
use terminal_size::terminal_size;

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

    let config_file = match args.config {
        Some(path) => Ok(path),
        None => get_config_path(),
    }
    .map_err(|e| {
        eprintln!("Error encountered: {e:?}");
        io::ErrorKind::NotFound
    })?;

    if !config_file.exists() {
        create_dir_all(config_file.parent().unwrap_or(Path::new("/")))
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

        write_default_config(&config_file).map_err(|e| {
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
    let end_date = args.end_date.unwrap_or(
        Local::now()
            .date_naive()
            .checked_add_days(Days::new(1))
            .expect(
                "
                How far in the future are you using this??
                ",
            ),
    );
    if start_date > end_date {
        eprintln!(
            "
            Invalid start and end dates. Start date falls later than the end date.
            "
        );
        return Err(Error::from(io::ErrorKind::InvalidInput));
    }

    let mut notes = args
        .notes
        .unwrap_or(PathBuf::from_str(&config.notes_folder).unwrap());
    let schedule = parse_sequence(&start_date, &end_date, &mut notes);

    let (terminal_size::Width(width), terminal_size::Height(_)) =
        terminal_size()
            .unwrap_or((terminal_size::Width(80), terminal_size::Height(0)));

    print_todos(&schedule.tbd_todos, &config.todos, width);
    print_comments(&schedule.comments, &config.comments, width);
    print_schedule(&schedule.events, &config.schedule, width);

    Ok(())
}
