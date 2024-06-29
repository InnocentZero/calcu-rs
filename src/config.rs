use std::{
    default::Default,
    env,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use derive_builder::Builder;
use log::{error, trace};
use serde::{Deserialize, Serialize};
use tabled::settings::{Style, Theme};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum TableStyle {
    Empty,
    Extended,
    Blank,
    Ascii,
    Psql,
    MarkDown,
    Modern,
    Sharp,
    Rounded,
    ModernRounded,
    Rst,
    Dots,
    AsciiRounded,
}

impl From<TableStyle> for Theme {
    fn from(value: TableStyle) -> Self {
        match value {
            TableStyle::Empty => Style::empty().into(),
            TableStyle::Extended => Style::extended().into(),
            TableStyle::Blank => Style::blank().into(),
            TableStyle::Ascii => Style::ascii().into(),
            TableStyle::Psql => Style::psql().into(),
            TableStyle::MarkDown => Style::markdown().into(),
            TableStyle::Modern => Style::modern().into(),
            TableStyle::Sharp => Style::sharp().into(),
            TableStyle::Rounded => Style::rounded().into(),
            TableStyle::ModernRounded => Style::modern_rounded().into(),
            TableStyle::Rst => Style::re_structured_text().into(),
            TableStyle::Dots => Style::dots().into(),
            TableStyle::AsciiRounded => Style::ascii_rounded().into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Alignment {
    Center,
    Left,
    Right,
}

impl From<Alignment> for tabled::settings::Alignment {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Center => Self::center(),
            Alignment::Left => Self::left(),
            Alignment::Right => Self::right(),
        }
    }
}

#[derive(Deserialize, Serialize, Builder, Debug)]
pub struct UpperConfig {
    #[builder(default = "String::from(\"~/notes\")")]
    pub notes_folder: String,
    #[builder(default = "NaiveDate::from_ymd_opt(2001, 01, 14).unwrap()")]
    pub start_date: NaiveDate,
    pub todos: TodoConfig,
    pub schedule: ScheduleConfig,
    pub comments: CommentConfig,
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone)]
pub struct TodoConfig {
    #[builder(default = "TableStyle::Empty")]
    pub table_style: TableStyle,
    #[builder(default = "(0, 0, 0, 0)")]
    pub margins: (usize, usize, usize, usize),
    #[builder(default = "Alignment::Center")]
    pub alignment: Alignment,
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone)]
pub struct ScheduleConfig {
    #[builder(default = "TableStyle::Empty")]
    pub table_style: TableStyle,
    #[builder(default = "(0, 0, 0, 0)")]
    pub margins: (usize, usize, usize, usize),
    #[builder(default = "Alignment::Center")]
    pub alignment: Alignment,
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone)]
pub struct CommentConfig {
    #[builder(default = "TableStyle::Empty")]
    pub table_style: TableStyle,
    #[builder(default = "(0, 0, 0, 0)")]
    pub margins: (usize, usize, usize, usize),
    #[builder(default = "Alignment::Center")]
    pub alignment: Alignment,
}

impl UpperConfig {
    pub fn try_parse(config_file: &PathBuf) -> Result<Self> {
        let mut buf = String::new();

        let config_file = File::open(config_file).context(
            "
            Failed to open the config file from the pathbuf.
            ",
        )?;

        let mut reader = BufReader::new(config_file);
        reader.read_to_string(&mut buf).context(
            "
            Failed to read the contents of the file into the buffer.
            ",
        )?;

        let config: UpperConfig = toml::from_str(buf.as_str())
            .context("Failed to parse the contents to toml")?;
        Ok(config)
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = if let Ok(config_dir) = env::var("CALCU_RS_HOME") {
        trace!("$CALCU_RS_HOME was defined. Using the value {config_dir}");
        Ok(config_dir)
    } else if let Ok(config_dir) = env::var("XDG_CONFIG") {
        trace!("$XDG_CONFIG was defined. Using the value {config_dir}");
        Ok(config_dir)
    } else if let Ok(home_dir) = env::var("HOME") {
        trace!("$HOME was defined. Using the value {home_dir}/.config");
        let config_dir = [home_dir.as_str(), ".config"].join("/");
        Ok(config_dir)
    } else {
        match env::var("USER") {
            Ok(user) => {
                trace!(
                    "$USER was defined. Using the value /home/{user}/.config"
                );
                let config_dir = ["/home", user.as_str(), ".config"].join("/");
                Ok(config_dir)
            }
            Err(e) => {
                error!("None of the environment variables were defined to appropriately determine config directory.");
                Err(e)
            }
        }
    };

    Ok([config_dir?.as_str(), "calcurs", "config.toml"]
        .join("/")
        .into())
}

pub fn write_default_config(config_path: &PathBuf) -> Result<()> {
    let mut config_file = File::create(config_path).context(
        "
        Failed to create or open file. It might be a permissions issue, 
        since the relevant directories were created already.
        ",
    )?;

    let default_todos = TodoConfigBuilder::default().build().unwrap();
    let default_schedule = ScheduleConfigBuilder::default().build().unwrap();
    let default_comments = CommentConfigBuilder::default().build().unwrap();

    let default_config = UpperConfigBuilder::default()
        .todos(default_todos)
        .schedule(default_schedule)
        .comments(default_comments)
        .build()
        .unwrap();
    trace!("Built default config");

    let toml = toml::to_string(&default_config).context(
        "
        Failed to serialize the configuration to TOML. Please report 
        the issue to the maintainer.
        ",
    )?;
    config_file.write_all(toml.as_bytes()).context(
        "
        Failed to write the default config to the file. 
        It could possibly be a permissions issue.
        ",
    )?;

    Ok(())
}
