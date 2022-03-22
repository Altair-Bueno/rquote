use std::error::Error;
use std::rc::Rc;

use reqwest::Client;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

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
    let state = use_async_with_options(
        async move { fetch_quote(client.as_ref()).await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(quote) = &state.data {
        let quote = quote.clone();
        let class = classes!("m-2");
        html! {
            <div class = {classes!("container")}>
                <QuoteComponent {quote} {class}/>
            </div>
        }
    } else if let Some(error) = &state.error {
        let severity = Severity::Danger;
        let error = error.clone();
        html! {<ErrorComponent {severity} {error}/>}
    } else { html! {<LoadingComponent/>} }
}