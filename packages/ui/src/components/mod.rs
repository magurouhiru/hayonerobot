use dioxus::prelude::*;

#[component]
pub fn Primitive() -> Element {
    rsx! {
        div { "ui-primitive" }
    }
}
