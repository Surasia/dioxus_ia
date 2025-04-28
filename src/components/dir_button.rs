use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use crate::config::Configuration;

#[component]
pub fn DirButton(config: Signal<Configuration>, error_occured: Signal<bool>) -> Element {
    #[cfg(not(target_arch = "wasm32"))]
    rsx! {
        div { class: "m-0 flex justify-center items-center mt-[40px]",
            h5 { "Output Folder: {config.read().output_path} " }
            button {
                class: "dirbutton",
                onclick: move |_| async move {
                    let out_path = AsyncFileDialog::new().pick_folder().await;
                    if let Some(out_path) = out_path {
                        let m = config
                            .write()
                            .modify_output_path(out_path.path().to_str().unwrap().to_string());
                        if m.is_err() {}
                    }
                },
                "Browse"
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    rsx! {}
}
