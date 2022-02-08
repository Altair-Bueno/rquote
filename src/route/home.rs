use std::error::Error;
use std::rc::Rc;

use reqwest::{Client, Error as ReqwestError};
use yew::prelude::*;

use crate::AnimechanQuote;
use crate::component::error::*;
use crate::component::loading::*;
use crate::component::quote::*;

#[derive(Debug)]
pub enum Message {
    Loading,
    Successful(Vec<AnimechanQuote>),
    Failed(Rc<ReqwestError>),
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
            Err(x) => Message::Failed(Rc::new(x)),
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
    fn failed_view(error: Rc<dyn Error>, _ctx: &Context<Self>) -> Html {
        let onclick = |_| todo!();
        let _ = html! {
            <button {onclick} class={classes!("btn","btn-light","text-dark")}>
                {"Reload"}
            </button>
        };
        html! {
            <ErrorComponent severity={Severity::Danger} {error}>
                //{reload_button}
            </ErrorComponent>
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
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            x => {
                self.quotes = x;
                true
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.quotes {
            Message::Loading => Home::loading_view(),
            Message::Successful(x) => Home::successful_view(x),
            Message::Failed(x) => Home::failed_view(x.clone(), ctx),
        }
    }
}