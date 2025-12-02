mod button;
mod card;
mod input;
mod label;
mod navbar;
mod switch;
mod tabs;

use crate::components::tabs::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs { default_value: "Card",
            TabList {
                TabTrigger { value: "Card", index: 0usize, "Card" }
                TabTrigger { value: "Button", index: 1usize, "Button" }
                TabTrigger { value: "Input", index: 2usize, "Input" }
                TabTrigger { value: "Label", index: 3usize, "Label" }
                TabTrigger { value: "Navbar", index: 4usize, "Navbar" }
                TabTrigger { value: "Switch", index: 5usize, "Switch" }
                TabTrigger { value: "Tabs", index: 6usize, "Tabs" }
            }
            TabContent { index: 0usize, value: "Card", card::Demo {} }
            TabContent { index: 1usize, value: "Button", button::Demo {} }
            TabContent { index: 2usize, value: "Input", input::Demo {} }
            TabContent { index: 3usize, value: "Label", label::Demo {} }
            TabContent { index: 4usize, value: "Navbar", navbar::Demo {} }
            TabContent { index: 5usize, value: "Switch", switch::Demo {} }
            TabContent { index: 6usize, value: "Tabs", tabs::Demo {} }
        }
    }
}
