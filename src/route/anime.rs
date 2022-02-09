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

#[derive(Properties, PartialEq, Clone)]
pub struct AnimeProp {
    pub title: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Anime {
    title: String,
    page: Option<u32>,
}

#[async_trait(? Send)]
impl ViewAsyncListComponent<AnimechanQuote> for Anime {
    async fn fetch_data(&self, client: Client) -> Message<AnimechanQuote> {
        let response = AnimechanQuote::get_quote_title(&client, &self.title, self.page).await;
        match response {
            Ok(x) => Message::Successful(x),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }

    fn successful_view(
        &self,
        ctx: &Context<AsyncListComponent<AnimechanQuote, Self>>,
        quotes: &[AnimechanQuote],
    ) -> Html {
        quotes
            .iter()
            .map(|x| html! {
                <QuoteComponent quote = {x.clone()} header = {false}/>
            })
            .collect()
    }
    fn failed_view(&self, ctx: &Context<AsyncListComponent<AnimechanQuote, Self>>, error: Rc<dyn Error>) -> Html {
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
    fn loading_view(&self, ctx: &Context<AsyncListComponent<AnimechanQuote, Self>>) -> Html {
        html! {<LoadingComponent/>}
    }
}

impl Component for Anime {
    type Message = ();
    type Properties = AnimeProp;

    fn create(ctx: &Context<Self>) -> Self {
        Anime { title: ctx.props().title.clone(), page: None }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = self.title.as_str();
        html! {
            <>
                <h1 class = {classes!("ms-3","my-3")}>
                    {title}
                    <small class = {classes!("text-muted","ms-3")}>{"Anime"}</small>
                </h1>
                <AsyncListComponent<AnimechanQuote,Self> provider={self.clone()}/>
            </>
        }
    }
}