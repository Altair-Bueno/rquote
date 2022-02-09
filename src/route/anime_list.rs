use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::async_list::*;
use crate::component::async_list::ViewAsyncListComponent;
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

    fn successful_view(&self, _ctx: &Context<AsyncListComponent<String, Self>>, quotes: &[String]) -> Html {
        let elements = quotes
            .iter()
            .map(|x| {
                self.format_element(_ctx, x)
            });
        html! {
            <ul class = {classes!("list-group","m-3")}>
                <h5>{for elements}</h5>
            </ul>
        }
    }
    fn format_element(&self, _ctx: &Context<AsyncListComponent<String, Self>>, element: &String) -> Html {
        let route = Route::Anime { title: element.clone() };
        html! {
            <li class = {classes!("list-group-item",)}>
                <Link<Route> to={route} classes={classes!("link-dark")}>
                    {element.clone()}
                </Link<Route>>
            </li>
        }
    }
}

impl Component for AnimeList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AnimeList
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <AsyncListComponent<String,Self> provider={self.clone()}/>
            </>
        }
    }
}