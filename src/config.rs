use anyhow::{anyhow, Result};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use std::{
    default::Default,
    env,
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

#[derive(Deserialize, Serialize, Builder, Debug)]
pub struct _Config {
    config: Config,
}

#[derive(Deserialize, Serialize, Builder, Debug, Clone)]
pub struct Config {
    data_folder: String,
    #[builder(default = "true")]
    show_keybindings: bool,
    #[builder(default = "CalView::Monthly")]
    default_cal_view: CalView,
    #[builder(default = "true")]
    delete_confirmation: bool,
    #[builder(default = "true")]
    quit_confirmation: bool,
    #[builder(default = "40")]
    right_pane_percent: u8,
    #[builder(default = "String::from(\"+\")")]
    event_icon: String,
    #[builder(default = "String::from(\">\")")]
    today_icon: String,
    #[builder(default = "String::from(\"⊕ \")")]
    task_icon: String,
    #[builder(default = "String::from(\" \")")]
    done_icon: String,
    #[builder(default = "String::from(\" \")")]
    deadline_icon: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Default, Debug)]
pub enum CalView {
    #[default]
    Monthly,
    Daily,
    Weekly,
    ThreeDay,
}

impl _Config {
    pub fn try_parse(config_file: &PathBuf) -> Result<Self> {
        let mut buf = String::new();
        let config_file = File::open(config_file)?;
        let mut reader = BufReader::new(config_file);
        reader.read_to_string(&mut buf)?;
        let config: _Config = toml::from_str(buf.as_str())?;
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
    let default_inner = ConfigBuilder::default()
        .data_folder(
            config_path
                .parent()
                .ok_or(anyhow!("Cannot find the directory out of the filepath."))?
                .to_str()
                .ok_or(anyhow!("Ideally shouldn't happen??"))?
                .into(),
        )
        .build()
        .unwrap();
    let default_config = _ConfigBuilder::default()
        .config(default_inner)
        .build()
        .unwrap();
    let toml = toml::to_string(&default_config)?;
    config_file.write_all(toml.as_bytes())?;
    Ok(())
}
