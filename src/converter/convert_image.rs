use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use image::{ImageFormat, ImageReader};

use crate::config::Configuration;

pub fn convert_image(source_path: &Path, requested_format: &str) -> Result<()> {
    let config = Configuration::load()?;
    let reader = BufReader::new(File::open(source_path)?);
    let format = ImageFormat::from_path(source_path)?;
    let output_format = ImageFormat::from_extension(requested_format);
    if let Some(output_format) = output_format {
        let mut image = ImageReader::new(reader);
        image.set_format(format);
        let dynamic_image = image.decode()?;
        let mut output_path = PathBuf::from(config.output_path);
        output_path.push(source_path.file_stem().unwrap());
        output_path.set_extension(output_format.extensions_str().first().unwrap());
        let mut writer = BufWriter::new(File::create(output_path)?);
        dynamic_image.write_to(&mut writer, output_format)?;
    } else {
        bail!("Couldn't find format!")
    }
    Ok(())
}
