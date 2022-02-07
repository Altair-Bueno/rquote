use yew::prelude::*;
use yew_router::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::footer::*;
use crate::context::{Context as RQuoteContext, Theme};
use crate::route::*;

// https://animechan.vercel.app/
mod animechan;
mod component;
mod context;
mod route;

#[function_component(Main)]
fn app() -> Html {
    let context = RQuoteContext::new(Theme::Light);
    let page_background_class = context.theme().get_background_class();
    html! {
        // TODO div does not fill the whole website, just it's elements
        <div class={classes!(page_background_class, "bg-gradient","flex-fill")}>
            <ContextProvider<RQuoteContext> context={context}>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </ContextProvider<RQuoteContext>>
            <FooterComponent/>
        </div>
    }
}

fn main() {
    yew::start_app::<Main>();
}
