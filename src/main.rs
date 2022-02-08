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

#[function_component(Main)]
fn app() -> Html {
    let client = ClientContext::new(Client::new());
    let theme = Theme::default();
    let page_background_class = theme.get_background_class();
    html! {
        // TODO div does not fill the whole website, just it's elements
        <div class={classes!(page_background_class, "bg-gradient","flex-fill")}>
            <ContextProvider<ClientContext> context={client}>
                <ContextProvider<Theme> context={theme}>
                    <BrowserRouter>
                        <Switch<Route> render={Switch::render(switch)} />
                    </BrowserRouter>
                </ContextProvider<Theme>>
            </ContextProvider<ClientContext>>
        </div>
    }
}

fn main() {
    yew::start_app::<Main>();
}
