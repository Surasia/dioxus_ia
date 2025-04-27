use std::path::Path;

use dioxus::{html::HasFileData, prelude::*};
use dioxus_logger::tracing;

use rfd::{AsyncFileDialog, FileHandle};

use crate::converter::convert_image::convert_image;

#[component]
pub fn UploadRectangle(
    format: Signal<String>,
    error_occured: Signal<bool>,
    error_details: Signal<String>,
) -> Element {
    let read_file = move |files: Vec<FileHandle>| async move {
        for file_name in &files {
            let res = convert_image(file_name.path(), &format.read());
            if let Err(res) = res {
                tracing::error!(
                    "Failed to convert image {}: {res:#?}",
                    file_name.file_name()
                );
                *error_details.write() = res.to_string();
                *error_occured.write() = true;
            }
        }
    };

    let read_drag_file = move |files: Vec<String>| async move {
        for file_name in &files {
            let res = convert_image(&Path::new(file_name), &format.read());
            if let Err(res) = res {
                *error_details.write() = res.to_string();
                *error_occured.write() = true;
            }
        }
    };

    rsx! {
        div {
            class: "rectangle grid x-screen place-items-center text-center relative",
            ondragover: move |evt| {
                evt.prevent_default();
            },
            ondragleave: move |_| {},
            ondrop: move |evt| async move {
                evt.prevent_default();
                if let Some(file_engine) = evt.files() {
                    println!("{:#?}", file_engine.files());
                    read_drag_file(file_engine.files()).await;
                }
            },
            button {
                class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer",
                onclick: move |_| async move {
                    #[cfg(not(target_os = "android"))]
                    let files = AsyncFileDialog::new()
                        .add_filter(
                            "Image Files",
                            &[
                                "png",
                                "jpg",
                                "jpeg",
                                "gif",
                                "webp",
                                "pbm",
                                "pam",
                                "ppm",
                                "pgm",
                                "tiff",
                                "tif",
                                "tga",
                                "dds",
                                "bmp",
                                "ico",
                                "hdr",
                                "exr",
                                "ff",
                                "avif",
                                "qoi",
                                "pcx",
                            ],
                        )
                        .pick_files()
                        .await;
                    if let Some(files) = files {
                        read_file(files).await;
                    }
                },
            }
            h1 { "Drag and Drop your image files!" }
        }
    }
}
