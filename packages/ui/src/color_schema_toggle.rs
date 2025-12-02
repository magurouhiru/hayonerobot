use dioxus::prelude::*;

use crate::components::switch::*;

#[component]
pub fn ColorSchemaToggle() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        Switch {
            checked: checked(),
            aria_label: "Switch Demo",
            on_checked_change: move |new_checked| {
                checked.set(new_checked);
            },
            SwitchThumb {}
        }
    }
}
