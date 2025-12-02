use dioxus::prelude::*;

#[component]
pub fn Showcase() -> Element {
    rsx! {
        ui::showcase::Demo {}
    }
}
