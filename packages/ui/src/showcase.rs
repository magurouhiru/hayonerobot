use dioxus::prelude::*;
mod card;
use crate::components::tabs::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs { default_value: "tab1".to_string(), horizontal: true,
            TabList {
                TabTrigger { value: "tab1".to_string(), index: 0usize, "Tab 1" }
                TabTrigger { value: "tab2".to_string(), index: 1usize, "Tab 2" }
                TabTrigger { value: "tab3".to_string(), index: 2usize, "Tab 3" }
                TabTrigger { value: "Card", index: 3usize, "Card" }
            }
            TabContent { index: 0usize, value: "tab1".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 1 Content"
                }
            }
            TabContent {
                index: 1usize,
                class: "tabs-content",
                value: "tab2".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 2 Content"
                }
            }
            TabContent { index: 2usize, value: "tab3".to_string(),
                div {
                    width: "100%",
                    height: "5rem",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Tab 3 Content"
                }
            }
            TabContent { index: 3usize, value: "Card", card::Demo {} }
        }
    }
}
