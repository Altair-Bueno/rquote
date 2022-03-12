use std::error::Error;
use std::rc::Rc;

use reqwest::Client;
use yew::prelude::*;
use yew_hooks::{use_async, UseAsyncHandle};

use rquote_component::button::*;
use rquote_component::error::*;
use rquote_component::loading::LoadingComponent;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;

async fn fetch_quote(client: &Client) -> Result<AnimechanQuote, Rc<dyn Error>> {
    let response = AnimechanQuote::get_random_quote(client).await;
    match response {
        Ok(quote) => Ok(quote),
        Err(error) => {
            let into_trait: Rc<dyn Error> = Rc::new(error);
            Err(into_trait)
        }
    }
}

#[function_component(Lucky)]
pub fn lucky() -> Html {
    let client: ClientWrapper = use_context().unwrap_or_default();
    let state = use_async(async move { fetch_quote(client.as_ref()).await });
    {
        let state = state.clone();
        use_effect_with_deps(move |_| {
            state.run();
            || ()
        }, ());
    }

    if state.loading {
        html! {<LoadingComponent/>}
    } else if let Some(quote) = &state.data {
        let quote = quote.clone();
        let class = classes!("m-3");
        html! {<QuoteComponent {quote} {class}/>}
    } else if let Some(error) = &state.error {
        let severity = Severity::Danger;
        let error = error.clone();
        html! {<ErrorComponent {severity} {error}/>}
    } else { Default::default() }
}