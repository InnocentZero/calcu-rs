use std::{
    default::Default,
    env,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use chrono::NaiveDate;
use derive_builder::Builder;
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
        let config_file = File::open(config_file)?;
        let mut reader = BufReader::new(config_file);
        reader.read_to_string(&mut buf)?;
        let config: UpperConfig = toml::from_str(buf.as_str())?;
        Ok(config)
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = if let Ok(config_dir) = env::var("CALCURSE_HOME") {
        Ok(config_dir)
    } else if let Ok(config_dir) = env::var("XDG_CONFIG") {
        Ok(config_dir)
    } else if let Ok(home_dir) = env::var("HOME") {
        let config_dir = [home_dir.as_str(), ".config"].join("/");
        Ok(config_dir)
    } else {
        match env::var("USER") {
            Ok(user) => {
                let config_dir = ["/home", user.as_str(), ".config"].join("/");
                Ok(config_dir)
            }
            Err(e) => Err(e),
        }
    };

    Ok([config_dir?.as_str(), "calcurs", "config.toml"]
        .join("/")
        .into())
}
pub fn write_default_config(config_path: &PathBuf) -> Result<()> {
    let mut config_file = File::create(config_path)?;

    let default_todos = TodoConfigBuilder::default().build().unwrap();
    let default_schedule = ScheduleConfigBuilder::default().build().unwrap();
    let default_comments = CommentConfigBuilder::default().build().unwrap();

    let default_config = UpperConfigBuilder::default()
        .todos(default_todos)
        .schedule(default_schedule)
        .comments(default_comments)
        .build()
        .unwrap();

    let toml = toml::to_string(&default_config)?;
    config_file.write_all(toml.as_bytes())?;
    Ok(())
}
