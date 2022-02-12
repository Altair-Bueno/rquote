use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::pager::*;
use rquote_component::Theme;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterProp {
    pub character: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterProvider {
    character: String,
    page: u32,
    client: ClientWrapper,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for CharacterProvider {
    async fn fetch_data(&self) -> Message<Vec<AnimechanQuote>> {
        let response =
            AnimechanQuote::get_quote_character(self.client.as_ref(), &self.character, Some(self.page)).await;
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
    let page = use_state(|| 0u32);
    let prev = {
        if *page == 0 {
            None
        } else {
            let page = page.clone();
            Some(Callback::from(move |_: MouseEvent| page.set(*page - 1)))
        }
    };
    let next = {
        let page = page.clone();
        Some(Callback::from(move |_: MouseEvent| page.set(*page + 1)))
    };
    let provider = CharacterProvider {
        page: *page,
        character: props.character.to_string(),
        client: use_context().unwrap_or_default(),
    };
    html! {
        <>
            <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                {title}
                <small class = {classes!("text-muted","ms-3")}>{"Character"}</small>
            </h1>
            <AsyncComponent<Vec<AnimechanQuote>,CharacterProvider>
                {provider}/>
            //<PagerComponent page = {*page} {prev} {next}/>
        </>
    }
}
