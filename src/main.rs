use yew::prelude::*;
use yew_router::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::context::Context as RQuoteContext;
use crate::route::Route;
use crate::route::switch;

// https://animechan.vercel.app/
mod animechan;
mod component;
mod context;
mod route;

#[function_component(Main)]
fn app() -> Html {
    let context = RQuoteContext::new();
    html! {
        <ContextProvider<RQuoteContext> context={context}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<RQuoteContext>>
    }
}

fn main() {
    yew::start_app::<Main>();
}
