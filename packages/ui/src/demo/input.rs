use crate::components::input::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut name = use_signal(String::new);
    let mut age = use_signal(String::new);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 1rem;",

            // Text input demo
            div {
                Input {
                    oninput: move |e: FormEvent| name.set(e.value()),
                    placeholder: "Enter your name",
                    value: name,
                }
                if !name.read().is_empty() {
                    p { id: "input-greeting", "Hello, {name}!" }
                }
            }

            // Number input demo
            div {
                Input {
                    r#type: "number",
                    oninput: move |e: FormEvent| age.set(e.value()),
                    placeholder: "Enter your age",
                    value: age,
                    min: "0",
                    max: "150",
                    step: "1",
                }
                if !age.read().is_empty() {
                    if let Ok(num) = age.read().parse::<i32>() {
                        p { id: "age-display", "You are {num} years old!" }
                        if num < 18 {
                            p { style: "color: var(--secondary-warning-color);", "You are a minor." }
                        } else if num >= 65 {
                            p { style: "color: var(--secondary-info-color);", "You are a senior." }
                        } else {
                            p { style: "color: var(--secondary-success-color);", "You are an adult." }
                        }
                    } else {
                        p { style: "color: var(--secondary-error-color);", "Invalid number" }
                    }
                }
            }
        }
    }
}
