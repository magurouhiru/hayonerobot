use dioxus::prelude::*;
use ui::settings_card::SettingsCard;

#[component]
pub fn Setting() -> Element {
    rsx! {
        div {
            style: "padding: 2rem; max-width: 800px; margin: 0 auto;",
            SettingsCard {}
        }
    }
}
