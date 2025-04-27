use dioxus::prelude::*;

#[component]
pub fn FormatSelector(format: Signal<String>) -> Element {
    rsx! {
        h5 { "Format: " }
        select {
            class: "formatpicker",
            onchange: move |evt| {
                if evt.data.value() == "Farbfeld" {
                    *format.write() = "ff".to_string();
                } else {
                    *format.write() = evt.data.value();
                }
            },
            option { id: "png", "PNG" }
            option { id: "jpg", "JPG" }
            option { id: "webp", "WEBP" }
            option { id: "avif", "AVIF" }
            option { id: "bmp", "BMP" }
            option { id: "dds", "DDS" }
            option { id: "ff", "Farbfeld" }
            option { id: "gif", "GIF" }
            option { id: "hdr", "HDR" }
            option { id: "ico", "ICO" }
            option { id: "exr", "EXR" }
            option { id: "pnm", "PNM" }
            option { id: "qoi", "QOI" }
            option { id: "tga", "TGA" }
            option { id: "tif", "TIF" }
        }
    }
}
