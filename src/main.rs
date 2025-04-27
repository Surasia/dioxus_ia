use components::{
    dir_button::DirButton, drag_drop::UploadRectangle, error::ErrorComponent,
    format_selector::FormatSelector,
};
use config::Configuration;
use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};
use dioxus_logger::tracing::Level;

mod components;
mod config;
mod converter;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
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
    let conf = Configuration::load();
    if let Err(ref err) = conf {
        *error_details.write() = err.to_string();
        *error_occured.write() = true
    };
    if *error_occured.read() {
        return rsx! {
            ErrorComponent { error_occured, error_details }
        };
    }
    let config = use_signal(|| conf.unwrap());
    rsx! {
        document::Stylesheet { href: CSS }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
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
