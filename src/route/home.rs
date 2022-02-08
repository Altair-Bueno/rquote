use reqwest::{Client, Error};
use yew::prelude::*;

use crate::AnimechanQuote;
use crate::component::loading::*;
use crate::component::quote::*;

#[derive(Debug)]
pub enum Message {
    Loading,
    Successful(Vec<AnimechanQuote>),
    Failed(reqwest::Error),
}

impl Default for Message {
    fn default() -> Self {
        Message::Loading
    }
}

#[derive(Debug, Default)]
pub struct Home {
    quotes: Message,
}

impl Home {
    async fn fetch_data(client: &Client) -> Message {
        let response = AnimechanQuote::get_10_random_quotes(&client).await;
        return match response {
            Ok(x) => Message::Successful(x),
            Err(x) => Message::Failed(x)
        };
    }

    fn loading_view() -> Html {
        html! {<LoadingComponent/>}
    }
    fn successful_view(quotes: &[AnimechanQuote]) -> Html {
        quotes
            .iter()
            .map(|x| html! {
                <QuoteComponent quote = {x.clone()}/>
            })
            .collect()
    }
    fn failed_view(error: &reqwest::Error) -> Html {
        html! {
            {"Something happened. Please refresh the page"}
        }
    }
}

impl Component for Home {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let client = ctx
            .link()
            .context::<crate::wrapper::ClientContext>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default()
            .take();
        ctx
            .link()
            .callback_future_once(|_| async move {
                Home::fetch_data(&client).await
            })
            .emit(());
        Home::default()
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.quotes = msg;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let client = ctx
            .link()
            .context::<crate::wrapper::ClientContext>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default()
            .take();
        match &self.quotes {
            Message::Loading => Home::loading_view(),
            Message::Successful(x) => Home::successful_view(x),
            Message::Failed(x) => Home::failed_view(x),
        }
    }
}