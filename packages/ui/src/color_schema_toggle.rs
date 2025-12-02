use dioxus::prelude::*;

use crate::components::switch::*;

#[component]
pub fn ColorSchemaToggle() -> Element {
    let mut is_dark = use_signal(|| false);

    // Initialize theme from localStorage on component mount
    use_effect(move || {
        spawn(async move {
            let result = document::eval(
                r#"
                const savedTheme = localStorage.getItem('theme');
                if (savedTheme) {
                    document.documentElement.setAttribute('data-theme', savedTheme);
                    return savedTheme === 'dark';
                } else {
                    // Default to system preference
                    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                    return prefersDark;
                }
                "#,
            )
            .await;

            if let Ok(value) = result {
                if let Some(is_dark_mode) = value.as_bool() {
                    is_dark.set(is_dark_mode);
                }
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./color_schema_toggle.css") }
        div { class: "color-schema-toggle-container",
            span { class: "color-schema-icon sun-icon", "☀️" }
            Switch {
                checked: is_dark(),
                aria_label: "Toggle Dark/Light Mode",
                on_checked_change: move |new_checked| {
                    is_dark.set(new_checked);

                    // Update theme in DOM and localStorage
                    let theme = if new_checked { "dark" } else { "light" };
                    spawn(async move {
                        let _ = document::eval(
                                &format!(
                                    "document.documentElement.setAttribute('data-theme', '{}');localStorage.setItem('theme', '{}');",
                                    theme,
                                    theme,
                                ),
                            )
                            .await;
                    });
                },
                SwitchThumb {}
            }
            span { class: "color-schema-icon moon-icon", "🌙" }
        }
    }
}
