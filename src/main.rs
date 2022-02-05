use yew::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::loading::Loading;
use crate::component::quote::{QuoteComponent, QuoteProp};

// https://animechan.vercel.app/
mod animechan;
mod component;

#[function_component(App)]
fn app() -> Html {
    let p = QuoteProp {
        header: true,
        quote: Box::new(AnimechanQuote::get_example()),
    };
    (0..3)
        .into_iter()
        .map(|_| p.clone())
        .map(|x| {
            html! {
                <QuoteComponent ..x/>
            }
        })
        .chain((0..2).map(|_| html! {
            <Loading/>
        }))
        .collect()
}

fn main() {
    yew::start_app::<App>();
}