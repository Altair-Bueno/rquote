use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::async_load::*;
use crate::component::async_load::ViewAsync;
use crate::component::list::*;
use crate::route::Route;

#[derive(Debug, PartialEq, Clone)]
pub struct AnimeList;

#[async_trait(? Send)]
impl ViewAsync<Vec<String>> for AnimeList {
    async fn fetch_data(&self, client: Client) -> Message<Vec<String>> {
        let response = AnimechanQuote::get_anime_list(&client).await;
        match response {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }
    fn successful_view(&self, ctx: &Context<AsyncComponent<Vec<String>, Self>>, element: Rc<Vec<String>>) -> Html {
        let formatted = element
            .iter()
            .map(|x| {
                let route = Route::Anime { title: x.clone() };
                html! {
                    <Link<Route> to={route} classes={classes!("link-dark")}>
                        {x.clone()}
                    </Link<Route>>
            }
            });
        html! {
            <ListComponent>
                {for formatted}
            </ListComponent>
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
            <AsyncComponent<Vec<String>,Self> provider={self.clone()}/>
        }
    }
}