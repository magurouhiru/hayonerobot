use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates local client-side echo (no server required).
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div { id: "echo",
            h4 { "Client Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| {
                    // Client-side echo - no server needed
                    response.set(event.value());
                },
            }

            if !response().is_empty() {
                p {
                    "Client echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}
