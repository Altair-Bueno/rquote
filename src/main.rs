use reqwest::Client;
use web_sys::{MediaQueryList, window, Window};
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
    ThemeChanged(Theme),
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
}

impl Component for Main {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // TODO watch for theme changes
        Main {
            theme: Main::get_current_theme(),
            client: Client::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ThemeChanged(new_theme) => {
                self.theme = new_theme;
                true
            },
            Message::Nop => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let client = ClientContext::new(self.client.clone());
        let theme = self.theme.clone();
        let page_background_class = theme.get_background_class();
        html! {
        // TODO div does not fill the whole website, just it's elements
        //<div class={classes!(page_background_class, "bg-gradient","flex-fill")}>
            <ContextProvider<ClientContext> context={client}>
                <ContextProvider<Theme> context={theme}>
                    <BrowserRouter>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </ContextProvider<Theme>>
            </ContextProvider<ClientContext>>
        //</div>
    }
    }
}

fn main() {
    yew::start_app::<Main>();
}
