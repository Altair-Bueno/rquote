use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::context::Theme;
use crate::route::*;
use crate::wrapper::ClientContext;

// https://animechan.vercel.app/
mod animechan;
mod component;
mod context;
mod route;
mod wrapper;

enum Message {
    // FIXME
    #[allow(unused)]
    ThemeChanged(Theme),
    // FIXME
    #[allow(unused)]
    Nop,
}

struct Main {
    theme: Theme,
    client: Client,
}

impl Main {
    fn get_current_theme() -> Theme {
        let prefers_dark = web_sys::window()
            .map(|window| {
                window.match_media("(prefers-color-scheme: dark)")
                    .map(|optional| {
                        optional.map(|x| x.matches())
                            .unwrap_or_default()
                    })
                    .unwrap_or_default()
            }).unwrap_or_default();
        if prefers_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
    fn change_body_theme(theme: &Theme) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    body.remove_attribute("class");
                    body.class_list().add_2(theme.get_background_class(), "bg-opacity-75");
                }
            }
        }
    }
}

impl Component for Main {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // TODO watch for theme changes
        let theme = Main::get_current_theme();
        Main::change_body_theme(&theme);
        Main {
            theme,
            client: Client::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ThemeChanged(new_theme) => {
                self.theme = new_theme;
                true
            },
            Message::Nop => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let client = ClientContext::new(self.client.clone());
        let theme = self.theme.clone();
        Main::change_body_theme(&theme);
        html! {
            <ContextProvider<ClientContext> context={client}>
                <ContextProvider<Theme> context={theme}>
                    <BrowserRouter>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </ContextProvider<Theme>>
            </ContextProvider<ClientContext>>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
