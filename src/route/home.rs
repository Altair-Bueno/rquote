use reqwest::Error;
use yew::prelude::*;

use crate::AnimechanQuote;
use crate::component::quote::*;

#[function_component(Home)]
pub fn home() -> Html {
    let client = use_context::<crate::context::Context>()
        .unwrap_or_default()
        .client()
        .clone();
    let quotes = use_state(|| Ok(vec![]));
    {
        let quotes = quotes.clone();
        let client = client.clone();
        use_effect_with_deps(
            move |_| {
                let client = client.clone();
                let quotes = quotes.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = AnimechanQuote::get_10_random_quotes(&client).await;
                    quotes.set(response);
                });
                || ()
            },
            (),
        );
    };
    let html = match quotes.as_ref() {
        Ok(quotes) => quotes
            .iter()
            .map(|x| {
                html! {
                    <QuoteComponent quote = {Box::new(x.clone())}/>
                }
            })
            .collect::<Html>(),
        Err(err) => {
            html! {
                <p>{format!("Something has happened: {err}")}</p>
            }
        }
    };
    html! {
        {html}
    }
}
