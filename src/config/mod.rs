use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
    str::FromStr,
};

#[derive(Serialize, Deserialize, Default, Props, Clone, PartialEq)]
pub struct Configuration {
    pub output_path: String,
}

impl Configuration {
    pub fn load() -> Result<Configuration> {
        let path = PathBuf::from_str("imager_config.toml")?;
        if !path.exists() {
            let mut f = File::create(&path).unwrap();
            let m = toml::to_string(&Configuration::default())?;
            f.write_all(m.as_bytes())?;
        }
        let mut file = File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(toml::from_str::<Configuration>(&buf)?)
    }

    pub fn modify_output_path(&mut self, val: String) -> Result<()> {
        self.output_path = val;
        let path = PathBuf::from_str("imager_config.toml")?;
        let mut file = BufWriter::new(File::create(&path)?);
        let m = toml::to_string(self)?;
        file.write_all(m.as_bytes())?;
        Ok(())
    }
}
