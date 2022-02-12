use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;

#[derive(Debug, PartialEq, Clone)]
struct HomeProvider {
    client: ClientWrapper,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for HomeProvider {
    async fn fetch_data(&self) -> Message<Vec<AnimechanQuote>> {
        let response = AnimechanQuote::get_10_random_quotes(self.client.as_ref()).await;
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
                    <QuoteComponent quote = {x.clone()}/>
                }
            })
            .collect()
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let provider = HomeProvider {
        client: use_context().unwrap_or_default()
    };
    html! {
        <AsyncComponent<Vec<AnimechanQuote>,HomeProvider> {provider}/>
    }
}
