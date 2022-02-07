use yew::prelude::*;

use crate::AnimechanQuote;
use crate::component::nav_bar::*;
use crate::component::quote::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterProp {
    pub character: String,
}

#[function_component(Character)]
pub fn home(character_prop: &CharacterProp) -> Html {
    let client = use_context::<crate::context::Context>()
        .unwrap_or_default()
        .client()
        .clone();
    let quotes = use_state(|| Ok(vec![]));
    let character = character_prop.character.to_string();
    {
        let quotes = quotes.clone();
        let client = client.clone();
        let character = character.clone();
        use_effect_with_deps(
            move |_| {
                let client = client.clone();
                let quotes = quotes.clone();
                let character = character.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response =
                        AnimechanQuote::get_quote_character(&client, character.as_str(), None)
                            .await;
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
                    <QuoteComponent quote = {x.clone()} footer={false}/>
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
