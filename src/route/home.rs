use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::async_load::*;
use crate::component::async_load::ViewAsync;
use crate::component::quote::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Home;

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for Home {
    async fn fetch_data(&self, client: Client) -> Message<Vec<AnimechanQuote>> {
        let response = AnimechanQuote::get_10_random_quotes(&client).await;
        match response {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }
    fn successful_view(&self, _ctx: &Context<AsyncComponent<Vec<AnimechanQuote>, Self>>, element: Rc<Vec<AnimechanQuote>>) -> Html {
        element
            .iter()
            .map(|x| html! {
                <QuoteComponent quote = {x.clone()}/>
            })
            .collect()
    }
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <AsyncComponent<Vec<AnimechanQuote>,Self> provider={self.clone()}/>
        }
    }
}