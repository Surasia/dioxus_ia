use components::{
    dir_button::DirButton, drag_drop::UploadRectangle, error::ErrorComponent,
    format_selector::FormatSelector,
};
use config::Configuration;
#[cfg(not(target_arch = "wasm32"))]
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

mod components;
mod config;
mod converter;

static CSS: Asset = asset!("/assets/main.css");
static TAILWIND: Asset = asset!("/assets/tailwind.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");
const _: Asset = asset!("/assets/IBMPlexMono.woff2");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_resizable(true)
                        .with_maximized(true)
                        .with_title("Imager"),
                )
                .with_menu(None)
                .with_disable_drag_drop_handler(false),
        )
        .launch(App);
    #[cfg(target_arch = "wasm32")]
    dioxus::launch(App);
}

#[component]
fn TitleDescription() -> Element {
    rsx! {
        div { class: "grid",
            p { class: "text-center text-[50px] mb-[10px]", "Imager" }
            p { class: "text-center text-[15px] mt-[0px]",
                "A simple image converter with no fuss, all done locally."
            }
        }
    }
}

#[component]
fn App() -> Element {
    let format = use_signal(|| String::from("png"));
    let mut error_occured = use_signal(|| false);
    let mut error_details = use_signal(|| String::from(""));
    #[cfg(not(target_arch = "wasm32"))]
    let conf = Configuration::load();
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(ref err) = conf {
        *error_details.write() = err.to_string();
        *error_occured.write() = true
    };
    if *error_occured.read() {
        return rsx! {
            ErrorComponent { error_occured, error_details }
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    let config = use_signal(|| conf.unwrap());
    #[cfg(target_arch = "wasm32")]
    let config = use_signal(|| Configuration::default());
    rsx! {
        document::Stylesheet { href: CSS }
        document::Stylesheet { href: TAILWIND }
        document::Link { rel: "icon", href: FAVICON }
        TitleDescription {}
        div { class: "self-center grid x-screen place-items-center mt-[50px]",
            UploadRectangle { format, error_occured, error_details }
            DirButton { config, error_occured }
            div { class: "m-0 flex items-center",
                FormatSelector { format }
            }
        }
    }
}
