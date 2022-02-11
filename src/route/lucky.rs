use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;

use rquote_component::async_load::{*, ViewAsync};
use rquote_core::AnimechanQuote;

use crate::custom::quote::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Lucky;

#[async_trait(? Send)]
impl ViewAsync<AnimechanQuote> for Lucky {
    async fn fetch_data(&self, client: Client) -> Message<AnimechanQuote> {
        match AnimechanQuote::get_random_quote(&client).await {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(x) => Message::Failed(Rc::new(x)),
        }
    }
    fn successful_view(&self, _ctx: &Context<AsyncComponent<AnimechanQuote, Self>>, element: Rc<AnimechanQuote>) -> Html {
        let quote = element.as_ref().clone();
        html! {
            <QuoteComponent {quote}/>
        }
    }
}

impl Component for Lucky {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Lucky
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <AsyncComponent<AnimechanQuote,Self> provider = {self.clone()}/>
        }
    }
}