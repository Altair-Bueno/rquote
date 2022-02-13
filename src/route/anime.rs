use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::button::*;
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
    page: u32,
    client: ClientWrapper,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<AnimechanQuote>> for AnimeProvider {
    async fn fetch_data(&self) -> Message<Vec<AnimechanQuote>> {
        let response = AnimechanQuote::get_quote_title(self.client.as_ref(), &self.title, Some(self.page)).await;
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
    let client = use_context::<ClientWrapper>()
        .unwrap_or_default();
    let title = props.title.as_str();
    let page = use_state(|| 0u32);
    let onclick: Callback<MouseEvent> = {
        let page = page.clone();
        move |_: MouseEvent| page.set(*page + 1)
    }.into();
    let async_component_list = (0..=*page)
        .map(|page| AnimeProvider {
            client: client.clone(),
            title: title.to_string(),
            page,
        })
        .map(|provider| html! {
            <AsyncComponent<Vec<AnimechanQuote>,AnimeProvider>
                {provider}/>
        });
    html! {
        <>
            <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                {title}
                <small class = {classes!("text-muted","ms-3")}>{"Anime"}</small>
            </h1>
            {for async_component_list}
            <div class="container">
                <div class="row">
                    <div class="col text-center">
                        <ButtonComponent {onclick} class ={classes!("text-center","my-2")}>
                            {"Load more"}
                        </ButtonComponent>
                    </div>
                </div>
            </div>
        </>
    }
}
