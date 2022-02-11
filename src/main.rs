use reqwest::Client;
use wasm_bindgen::JsCast;
use web_sys::MediaQueryList;
use yew::prelude::*;
use yew_router::prelude::*;

use rquote_component::Theme;
use rquote_core::wrapper::ClientWrapper;

use crate::route::*;

mod custom;
mod route;

enum Message {
    ThemeChanged(Theme),
}

struct Main {
    theme: Theme,
    client: Client,
}

impl Main {
    fn get_current_theme() -> Theme {
        let prefers_dark = if let Some(window) = web_sys::window() {
            if let Ok(option) = window.match_media("(prefers-color-scheme: dark)") {
                if let Some(media_query) = option {
                    media_query.matches()
                } else { Default::default() }
            } else { Default::default() }
        } else { Default::default() };
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
                    let _ = body.remove_attribute("class");
                    let _ = body
                        .class_list()
                        .add_2(theme.get_background_class(), "bg-opacity-75");
                }
            }
        }
    }
    fn set_theme_listener(callback: Callback<Theme>) {
        if let Some(window) = web_sys::window() {
            if let Ok(option) = window.match_media("(prefers-color-scheme: dark)") {
                if let Some(media_query) = option {
                    let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |media_query: MediaQueryList| {
                        let theme = if media_query.matches() { Theme::Dark } else { Theme::Light };
                        callback.emit(theme);
                    }) as Box<dyn Fn(_)>);
                    let _ = media_query.add_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();
                }
            }
        };
    }
}

impl Component for Main {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let theme = Main::get_current_theme();
        let callback: Callback<Theme> = ctx
            .link()
            .callback(|theme: Theme| Message::ThemeChanged(theme));
        Main::set_theme_listener(callback);
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
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let client = ClientWrapper::new(self.client.clone());
        let theme = self.theme.clone();
        Main::change_body_theme(&theme);
        html! {
            <ContextProvider<ClientWrapper> context={client}>
                <ContextProvider<Theme> context={theme}>
                    <BrowserRouter>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </ContextProvider<Theme>>
            </ContextProvider<ClientWrapper>>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
