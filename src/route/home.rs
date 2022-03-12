use std::error::Error;
use std::rc::Rc;

use reqwest::Client;
use yew::prelude::*;
use yew_hooks::use_async;

use rquote_component::error::*;
use rquote_component::loading::*;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;

async fn fetch_data(client: &Client) -> Result<Vec<AnimechanQuote>, Rc<dyn Error>> {
    let response = AnimechanQuote::get_10_random_quotes(client).await;
    match response {
        Ok(quote) => Ok(quote),
        Err(error) => {
            let into_trait: Rc<dyn Error> = Rc::new(error);
            Err(into_trait)
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let client: ClientWrapper = use_context().unwrap_or_default();
    let state = use_async(async move { fetch_data(client.as_ref()).await });
    {
        let state = state.clone();
        use_effect_with_deps(move |_| {
            state.run();
            || ()
        }, ());
    }
    if state.loading {
        html! {<LoadingComponent/>}
    } else if let Some(quote_list) = &state.data {
        quote_list.iter().map(|quote| html! {
            <QuoteComponent quote = {quote.clone()} class = {classes!("m-3")}/>
        }).collect()
    } else if let Some(error) = &state.error {
        let severity = Severity::Danger;
        let error = error.clone();
        html! {<ErrorComponent {severity} {error}/>}
    } else { Default::default() }
}
