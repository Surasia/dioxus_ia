use dioxus::{
    html::{FileEngine, HasFileData},
    prelude::*,
};
use std::sync::Arc;

use crate::converter::convert_image::convert_image;

#[component]
pub fn UploadRectangle(
    format: Signal<String>,
    error_occured: Signal<bool>,
    error_details: Signal<String>,
) -> Element {
    let read_file = move |files: Option<Arc<dyn FileEngine>>| async move {
        if let Some(files) = files {
            let m = convert_image(files, &format.read()).await;
            if let Err(m) = m {
                *error_occured.write() = true;
                *error_details.write() = m.to_string();
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
                read_file(evt.files()).await;
            },
            input {
                accept: ".png, .jpg, .jpeg, .gif, .webp, .pbm, .pam, .ppm, .pgm, .tiff, .tif, .tga, .dds, .bmp, .ico, .hdr, .exr, .ff, .avif, .qoi, .pcx",
                r#type: "file",
                multiple: true,
                class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer",
                onchange: move |evt| async move {
                    read_file(evt.files()).await;
                },
            }
            h1 { "Drag and Drop your image files!" }
        }
    }
}
