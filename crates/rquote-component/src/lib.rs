//! # `rquote-component`
//!
//! This library provides generic components created with Yew's framework and
//! styled with Bootstrap
//! # Available [ContextProvider](yew::prelude::ContextProvider) options
//!
//! - [Theme](crate::Theme): Change component theme
//!

pub mod error;
pub mod footer;
pub mod list;
pub mod loading;
pub mod nav_bar;
pub mod not_found;
pub mod pager;
pub mod search_bar;
pub mod switch;
pub mod button;

/// Provides different flavours for theming rquote's components
#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    /// Light flavour
    Light,
    /// Dark flavour
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

impl Theme {
    pub fn get_navbar_class(&self) -> &'static str {
        match self {
            Theme::Dark => "navbar-dark",
            Theme::Light => "navbar-light",
        }
    }
    pub fn get_background_class(&self) -> &'static str {
        match self {
            Theme::Light => "bg-light",
            Theme::Dark => "bg-dark",
        }
    }
    pub fn get_text_class(&self) -> &'static str {
        match self {
            Theme::Dark => "text-light",
            Theme::Light => "text-dark",
        }
    }
    pub fn get_secondary_text_class(&self) -> &'static str {
        match self {
            Theme::Light => "text-muted",
            Theme::Dark => "text-light",
        }
    }
    pub fn get_link_class(&self) -> &'static str {
        match self {
            Theme::Dark => "link-light",
            Theme::Light => "link-dark",
        }
    }
    pub fn get_button_class(&self) -> &'static str {
        match self {
            Theme::Dark => "btn-light",
            Theme::Light => "btn-dark"
        }
    }
}
