use anyhow::Result;
use cross_xdg::BaseDirs;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::{BufWriter, Read, Write},
};

#[derive(Serialize, Deserialize, Default, Props, Clone, PartialEq)]
pub struct Configuration {
    pub output_path: String,
}

impl Configuration {
    pub fn load() -> Result<Configuration> {
        let base_dirs = BaseDirs::new()?;
        let mut config_home = base_dirs.config_home().to_path_buf();
        config_home.push("imager");
        config_home.push("imager_config.toml");
        if !config_home.exists() {
            create_dir_all(config_home.parent().unwrap())?;
            let mut f = File::create(&config_home).unwrap();
            let m = toml::to_string(&Configuration::default())?;
            f.write_all(m.as_bytes())?;
        }
        let mut file = File::open(&config_home)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(toml::from_str::<Configuration>(&buf)?)
    }

    pub fn modify_output_path(&mut self, val: String) -> Result<()> {
        self.output_path = val;
        let base_dirs = BaseDirs::new()?;
        let mut config_home = base_dirs.config_home().to_path_buf();
        config_home.push("imager");
        config_home.push("imager_config.toml");
        let mut file = BufWriter::new(File::create(&config_home)?);
        let m = toml::to_string(self)?;
        file.write_all(m.as_bytes())?;
        Ok(())
    }
}
