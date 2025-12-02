//! This crate contains all shared UI for the workspace.
use dioxus::prelude::*;
pub const THEME_CSS: Asset = asset!("../assets/dx-components-theme.css");

pub mod color_schema_toggle;
pub mod components;
pub mod showcase;
