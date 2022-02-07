use reqwest::Client;

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn get_navbar_class(&self) -> Vec<&'static str> {
        match self {
            Theme::Dark => vec!["navbar-dark", "bg-dark"],
            Theme::Light => vec!["navbar-light", "bg-light"]
        }
    }
}

#[derive(Clone, Debug)]
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
