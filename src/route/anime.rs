use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::Theme;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AnimeProp {
    pub title: String,
}

#[derive(Debug, PartialEq, Clone)]
struct AnimeProvider {
    title: String,
    page: Option<u32>,
    client: ClientWrapper,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for AnimeProvider {
    async fn fetch_data(&self) -> Message<Vec<AnimechanQuote>> {
        let response = AnimechanQuote::get_quote_title(self.client.as_ref(), &self.title, self.page).await;
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
                    <QuoteComponent quote = {x.clone()} header = {false}/>
                }
            })
            .collect()
    }
}

#[function_component(Anime)]
pub fn view(props: &AnimeProp) -> Html {
    let theme = use_context::<Theme>().unwrap_or_default();
    let title = props.title.as_str();
    let provider = {
        let title = title.to_string();
        let page = None;
        let client = use_context().unwrap_or_default();
        use_state(|| AnimeProvider { title, page, client })
    };
    html! {
        <>
            <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                {title}
                <small class = {classes!("text-muted","ms-3")}>{"Anime"}</small>
            </h1>
            <AsyncComponent<Vec<AnimechanQuote>,AnimeProvider> provider= {provider.deref().clone()}/>
        </>
    }
}
