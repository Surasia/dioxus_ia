use dioxus::prelude::*;

#[component]
pub fn ErrorComponent(error_occured: Signal<bool>, error_details: Signal<String>) -> Element {
    rsx! {
        div { class: "self-center grid place-items-center",
            h1 { background: "red", "An error occured while writing some of the files!" }
            h5 { "{error_details.read()}" }
            button {
                onclick: move |_| async move {
                    *error_occured.write() = false;
                },
                class: "formatpicker",
                "Ignore"
            }
        }
    }
}
