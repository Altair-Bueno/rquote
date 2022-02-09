use std::error::Error;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::async_list::*;
use crate::component::async_list::ViewAsyncListComponent;
use crate::component::error::*;
use crate::component::loading::*;
use crate::component::quote::*;
use crate::route::Route;

#[derive(Debug, PartialEq, Clone)]
pub struct AnimeList;

#[async_trait(? Send)]
impl ViewAsyncListComponent<String> for AnimeList {
    async fn fetch_data(&self, client: Client) -> Message<String> {
        let response = AnimechanQuote::get_anime_list(&client).await;
        match response {
            Ok(x) => Message::Successful(x),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }

    fn successful_view(&self, ctx: &Context<AsyncListComponent<String, Self>>, quotes: &[String]) -> Html {
        let elements = quotes
            .iter()
            .map(|x| {
                let route = Route::Anime { title: x.clone() };
                html! {
                <li class = {classes!("list-group-item",)}>
                    <Link<Route> to={route} classes={classes!("link-dark")}>
                        {x}
                    </Link<Route>>
                </li>
            }
            });
        html! {
            <ul class = {classes!("list-group","m-3")}>
                {for elements}
            </ul>
        }
    }

    fn failed_view(&self, ctx: &Context<AsyncListComponent<String, Self>>, error: Rc<dyn Error>) -> Html {
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

    fn loading_view(&self, ctx: &Context<AsyncListComponent<String, Self>>) -> Html {
        html! {<LoadingComponent/>}
    }
}

impl Component for AnimeList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AnimeList
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <AsyncListComponent<String,Self> provider={self.clone()}/>
            </>
        }
    }
}