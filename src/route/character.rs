use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::Theme;
use rquote_core::AnimechanQuote;

use crate::custom::quote::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterProp {
    pub character: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterProvider {
    character: String,
    page: Option<u32>,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for CharacterProvider {
    async fn fetch_data(&self, client: Client) -> Message<Vec<AnimechanQuote>> {
        let response =
            AnimechanQuote::get_quote_character(&client, &self.character, self.page).await;
        match response {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }
    fn successful_view(
        &self,
        _ctx: &Context<AsyncComponent<Vec<AnimechanQuote>, Self>>,
        element: Rc<Vec<AnimechanQuote>>,
    ) -> Html {
        element
            .iter()
            .map(|x| {
                html! {
                    <QuoteComponent quote = {x.clone()} footer = {false}/>
                }
            })
            .collect()
    }
}

#[function_component(Character)]
pub fn character(props: &CharacterProp) -> Html {
    let theme = use_context::<Theme>()
        .unwrap_or_default();
    let title = props.character.as_str();
    let provider = {
        let page = None;
        let character = title.to_string();
        use_state(|| CharacterProvider { page, character })
    };
    html! {
        <>
            <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                {title}
                <small class = {classes!("text-muted","ms-3")}>{"Character"}</small>
            </h1>
            <AsyncComponent<Vec<AnimechanQuote>,CharacterProvider> provider={provider.deref().clone()}/>
        </>
    }
}
