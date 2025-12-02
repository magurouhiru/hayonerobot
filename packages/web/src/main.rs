use dioxus::prelude::*;

mod views;
use ui::{color_schema_toggle::ColorSchemaToggle, components::navbar::*, THEME_CSS};
use views::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: THEME_CSS }
        Navbar {
            NavbarItem { index: 0usize, value: "Home", to: Route::Home {}, "Home" }
            NavbarItem { index: 1usize, value: "Showcase", to: Route::Showcase {}, "Showcase" }
        }
        ColorSchemaToggle {}
        Outlet::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/showcase")]
    Showcase {},
}
