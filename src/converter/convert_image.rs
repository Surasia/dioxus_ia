use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
#[cfg(target_arch = "wasm32")]
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{bail, Result};
#[cfg(target_arch = "wasm32")]
use dioxus::document::eval;
use dioxus::html::FileEngine;
#[cfg(target_arch = "wasm32")]
use dioxus_logger::tracing;
use image::{ImageFormat, ImageReader};

use crate::config::Configuration;

#[cfg(target_arch = "wasm32")]
const DOWNLOAD_JS: &str = r#"
            let value = await dioxus.recv();
            let filename = await dioxus.recv();
            let file = new File([new Uint8Array(value)], filename, {type: ''});
            const a = document.createElement("a");
            const url = URL.createObjectURL(file);
            a.href = url;
            a.download = file.name;
            a.click();
            URL.revokeObjectURL(url);
            "#;

pub async fn convert_image(
    source_paths: Arc<dyn FileEngine>,
    requested_format: &str,
) -> Result<()> {
    for source_path in &source_paths.files() {
        let format = ImageFormat::from_path(source_path)?;
        let output_format = ImageFormat::from_extension(requested_format);
        if let Some(output_format) = output_format {
            let mut image = ImageReader::new(Cursor::new(
                source_paths.read_file(source_path).await.unwrap(),
            ));
            image.set_format(format);
            let dynamic_image = image.decode()?;
            let mut buf = Cursor::new(Vec::new());
            dynamic_image.write_to(&mut buf, output_format)?;
            #[cfg(target_arch = "wasm32")]
            {
                let mut filename = PathBuf::from_str(source_path)?;
                filename.set_extension(output_format.extensions_str()[0]);
                let m = eval(DOWNLOAD_JS);
                m.send(buf.into_inner())?;
                m.send(filename.to_str().unwrap())?;
                let err = m.await;
                if let Err(err) = err {
                    tracing::error!("Communication Error occured: {}", err.to_string());
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let config = Configuration::load()?;
                let mut filepath = PathBuf::from_str(&config.output_path)?;
                filepath.set_file_name(source_path);
                filepath.set_extension(output_format.extensions_str()[0]);
                let mut writer = BufWriter::new(File::create(filepath)?);
                writer.write_all(&buf.into_inner())?;
            }
        } else {
            bail!("Couldn't find format!")
        }
    }
    Ok(())
}
