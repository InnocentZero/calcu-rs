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
use env_logger::Env;
use log::{error, info, warn};

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
    env_logger::Builder::from_env(Env::default().default_filter_or("off"))
        .init();
    let args = Args::parse();

    let config_file = match args.config {
        Some(path) => Ok(path),
        None => {
            info!("config path not supplied through arguments. Reading from default path");
            get_config_path()
        },
    }
    .map_err(|e| {
        eprintln!("Error encountered: {e:?}");
        io::ErrorKind::NotFound
    })?;

    if !config_file.exists() {
        create_dir_all(config_file.parent().unwrap_or(Path::new("/")))
            .map_err(|e| {
                error!("Failed to create parent directories");
                error!("While unlikely, it may be possible that their was no parent of the config file.");
                error!("{e:?}");
                io::ErrorKind::NotFound
            })?;

        let file = File::create(&config_file).map_err(|e| {
            error!("Error occured: {e:?}");
            io::ErrorKind::NotFound
        })?;

        file.sync_all().map_err(|e| {
            error!("Error occured: {e:?}");
            io::ErrorKind::InvalidData
        })?;

        write_default_config(&config_file).map_err(|e| {
            error!("Error occured while writing the default config.");
            error!("{e:?}");
            io::ErrorKind::InvalidInput
        })?;
    }

    let config = UpperConfig::try_parse(&config_file).map_err(|e| {
        error!("Error occured in reading config file!");
        error!("{e:?}");
        io::ErrorKind::InvalidData
    })?;

    let start_date = args.start_date.unwrap_or(config.start_date);
    let end_date = args.end_date.unwrap_or(
        Local::now()
            .date_naive()
            .checked_add_days(Days::new(1))
            .unwrap_or_else(|| {
                error!("How far in the future are you using this??");
                panic!();
            }),
    );
    if start_date > end_date {
        error!("Invalid start and end dates. Start date falls later than the end date.");
        return Err(Error::from(io::ErrorKind::InvalidInput));
    }

    let mut notes = args.notes.unwrap_or({
        info!("Notes not provided through cli. Falling back to config");
        PathBuf::from_str(&config.notes_folder).unwrap()
    });
    let schedule = parse_sequence(&start_date, &end_date, &mut notes);

    let size = termsize::get().unwrap_or_else(|| {
        warn!("Terminal size not found.");
        warn!("This is possibly either a TTY or an error");
        termsize::Size { rows: 0, cols: 80 }
    });

    match args.command {
        Commands::Todo => {
            print_todos(&schedule.tbd_todos, &config.todos, size.cols)
        }
        Commands::Schedule => {
            print_schedule(&schedule.events, &config.schedule, size.cols)
        }
        Commands::Logs => {
            print_comments(&schedule.comments, &config.comments, size.cols)
        }
    }

    Ok(())
}
