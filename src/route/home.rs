use std::error::Error;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::async_list::*;
use crate::component::async_list::ViewAsyncListComponent;
use crate::component::error::*;
use crate::component::loading::*;
use crate::component::quote::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Home;

#[async_trait(? Send)]
impl ViewAsyncListComponent<AnimechanQuote> for Home {
    async fn fetch_data(&self, client: Client) -> Message<AnimechanQuote> {
        let response = AnimechanQuote::get_10_random_quotes(&client).await;
        match response {
            Ok(x) => Message::Successful(x),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }

    fn successful_view(
        &self,
        _ctx: &Context<AsyncListComponent<AnimechanQuote, Self>>,
        quotes: &[AnimechanQuote],
    ) -> Html {
        quotes
            .iter()
            .map(|x| html! {
                <QuoteComponent quote = {x.clone()}/>
            })
            .collect()
    }
    fn failed_view(&self, _ctx: &Context<AsyncListComponent<AnimechanQuote, Self>>, error: Rc<dyn Error>) -> Html {
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
    fn loading_view(&self, _ctx: &Context<AsyncListComponent<AnimechanQuote, Self>>) -> Html {
        html! {<LoadingComponent/>}
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
            <AsyncListComponent<AnimechanQuote,Self> provider={self.clone()}/>
        }
    }
}