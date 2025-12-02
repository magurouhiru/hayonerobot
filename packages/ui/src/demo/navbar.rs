use crate::components::navbar::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "navbar-example",
            Navbar { aria_label: "Components",
                NavbarNav { index: 0usize,
                    NavbarTrigger { "Inputs" }
                    NavbarContent { class: "navbar-content",
                        NavbarItem {
                            index: 0usize,
                            value: "calendar".to_string(),
                            to: "#",
                            "Calendar"
                        }
                        NavbarItem {
                            index: 1usize,
                            value: "slider".to_string(),
                            to: "#",
                            "Slider"
                        }
                        NavbarItem {
                            index: 2usize,
                            value: "checkbox".to_string(),
                            to: "#",
                            "Checkbox"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: "#",
                            "Radio Group"
                        }
                    }
                }
                NavbarNav { index: 1usize,
                    NavbarTrigger { "Information" }
                    NavbarContent {
                        NavbarItem {
                            index: 0usize,
                            value: "toast".to_string(),
                            to: "#",
                            "Toast"
                        }
                        NavbarItem {
                            index: 1usize,
                            value: "tabs".to_string(),
                            to: "#",
                            "Tabs"
                        }
                        NavbarItem {
                            index: 2usize,
                            value: "dialog".to_string(),
                            to: "#",
                            "Dialog"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "alert_dialog".to_string(),
                            to: "#",
                            "Alert Dialog"
                        }
                        NavbarItem {
                            index: 4usize,
                            value: "tooltip".to_string(),
                            to: "#",
                            "Tooltip"
                        }
                    }
                }
                NavbarItem { index: 2usize, value: "home".to_string(), to: "#", "Home" }
            }
        }
    }
}
