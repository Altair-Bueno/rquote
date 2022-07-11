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
    /// Queries the browser for the user's preferred color scheme. Default
    /// otherwise
    fn get_current_theme() -> Theme {
        fn inner() -> Option<Theme> {
            let window = web_sys::window()?;
            let media_query = window.match_media("(prefers-color-scheme: dark)").ok()??;
            let theme = if media_query.matches() { Theme::Dark } else { Theme::Light };

            Some(theme)
        }
        inner().unwrap_or_default()
    }
    /// Changes the theme for all body tags on the DOM
    fn change_body_theme(theme: &Theme) {
        fn inner(theme:&Theme) -> Option<()> {
            let body = web_sys::window()?.document()?.body()?;
            let _ = body.remove_attribute("class");
            let _ = body
                .class_list()
                .add_2(theme.get_background_class(), "bg-opacity-75");
            
                Some(())
        }

        inner(theme);
    }
    /// Adds an EventListener to a `web_sys::Window::match_media("(prefers-color-scheme: dark)")`
    fn set_theme_listener(callback: Callback<Theme>) {
        fn inner(callback: Callback<Theme>) -> Option<()>{
            let window = web_sys::window()?;
            let media_query = window.match_media("(prefers-color-scheme: dark)").ok()??;
            let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move | media_query:MediaQueryList| {
                let theme = if media_query.matches() { Theme::Dark } else { Theme::Light };
                callback.emit(theme);
            }) as Box<dyn Fn(_)>);
            let _ = media_query.add_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
            Some(())
        }
        inner(callback);
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
