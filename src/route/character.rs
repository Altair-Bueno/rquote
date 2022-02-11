use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::pager::*;
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
    page: u32,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for CharacterProvider {
    async fn fetch_data(&self, client: Client) -> Message<Vec<AnimechanQuote>> {
        let response =
            AnimechanQuote::get_quote_character(&client, &self.character, Some(self.page)).await;
        match response {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }
    fn successful_view(
        &self,
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
        let page = 0;
        let character = title.to_string();
        use_state(|| CharacterProvider { page, character })
    };
    let prev = {
        if provider.deref().page == 0 {
            None
        } else {
            let provider = provider.clone();
            Some(Callback::from(move |_: MouseEvent| provider.set(CharacterProvider {
                page: provider.page - 1,
                ..provider.deref().clone()
            })))
        }
    };
    let next = {
        let provider = provider.clone();
        Some(Callback::from(move |_: MouseEvent| provider.set(CharacterProvider {
            page: provider.page + 1,
            ..provider.deref().clone()
        })))
    };
    html! {
        <>
            <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                {title}
                <small class = {classes!("text-muted","ms-3")}>{"Character"}</small>
            </h1>
            <AsyncComponent<Vec<AnimechanQuote>,CharacterProvider> provider={provider.deref().clone()}/>
            //<PagerComponent page = {provider.page} {prev} {next}/>
        </>
    }
}
