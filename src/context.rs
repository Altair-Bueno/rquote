use reqwest::Client;

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
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
            Theme::Light => "navbar-light"
        }
    }
    pub fn get_background_class(&self) -> &'static str {
        match self {
            Theme::Light => "bg-light",
            Theme::Dark => "bg-dark"
        }
    }
    pub fn get_text_class(&self) -> &'static str {
        match self {
            Theme::Dark => "text-light",
            Theme::Light => "text-dark",
        }
    }
    pub fn get_link_class(&self) -> &'static str {
        match self {
            Theme::Dark => "link-light",
            Theme::Light => "link-dark"
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    client: Client,
    theme: Theme,
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
    }
}

impl Context {
    pub fn new(theme: Theme) -> Context {
        Context {
            client: Client::new(),
            theme,
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}
