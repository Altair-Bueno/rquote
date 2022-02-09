use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::async_load::*;
use crate::component::async_load::ViewAsync;
use crate::component::quote::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterProp {
    pub character: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Character {
    character: String,
    page: Option<u32>,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for Character {
    async fn fetch_data(&self, client: Client) -> Message<Vec<AnimechanQuote>> {
        let response = AnimechanQuote::get_quote_character(&client, &self.character, self.page).await;
        match response {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }
    fn successful_view(&self, _ctx: &Context<AsyncComponent<Vec<AnimechanQuote>, Self>>, element: Rc<Vec<AnimechanQuote>>) -> Html {
        element
            .iter()
            .map(|x| html! {
                <QuoteComponent quote = {x.clone()} footer = {false}/>
            })
            .collect()
    }
}

impl Component for Character {
    type Message = ();
    type Properties = CharacterProp;

    fn create(ctx: &Context<Self>) -> Self {
        Character { character: ctx.props().character.clone(), page: None }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = ctx
            .link()
            .context::<crate::context::Theme>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default();
        let title = self.character.as_str();
        html! {
            <>
                <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                    {title}
                    <small class = {classes!("text-muted","ms-3")}>{"Character"}</small>
                </h1>
                <AsyncComponent<Vec<AnimechanQuote>,Self> provider={self.clone()}/>
            </>
        }
    }
}