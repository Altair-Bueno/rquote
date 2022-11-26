use std::error::Error;
use std::rc::Rc;

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use rquote_component::error::*;
use rquote_component::loading::*;
use rquote_core::AnimechanQuote;

use crate::custom::quote::*;

async fn fetch_data() -> Result<Vec<AnimechanQuote>, Rc<dyn Error>> {
    let response = AnimechanQuote::get_10_random_quotes().await;
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
    let state = use_async_with_options(
        async move { fetch_data().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(quote_list) = &state.data {
        let list: Vec<_> = quote_list
            .iter()
            .map(|quote| {
                html! {
                    <div class = "m-2">
                        <QuoteComponent quote = {quote.clone()} />
                    </div>
                }
            })
            .collect();
        html! {
            <div class = "container">
                {list}
            </div>
        }
    } else if let Some(error) = &state.error {
        let severity = Severity::Danger;
        let error = error.clone();
        html! {<ErrorComponent {severity} {error}/>}
    } else {
        html! {<LoadingComponent/>}
    }
}
