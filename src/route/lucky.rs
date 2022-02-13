use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use rquote_component::async_load::{*, ViewAsync};
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;

#[derive(Debug, PartialEq, Clone)]
struct LuckyProvider {
    client: ClientWrapper,
}

#[async_trait(? Send)]
impl ViewAsync<AnimechanQuote> for LuckyProvider {
    async fn fetch_data(&self) -> Message<AnimechanQuote> {
        match AnimechanQuote::get_random_quote(self.client.as_ref()).await {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(x) => Message::Failed(Rc::new(x)),
        }
    }
    fn successful_view(&self, element: Rc<AnimechanQuote>) -> Html {
        let quote = element.as_ref().clone();
        html! {
            <QuoteComponent {quote} class = {classes!("m-3")}/>
        }
    }
}

#[function_component(Lucky)]
pub fn lucky() -> Html {
    let provider = LuckyProvider {
        client: use_context().unwrap_or_default(),
    };
    html! {
        <AsyncComponent<AnimechanQuote,LuckyProvider> {provider}/>
    }
}