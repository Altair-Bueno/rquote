use std::rc::Rc;

use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use reqwest::Client;
use yew::prelude::*;
use yew_router::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::list::*;
use rquote_component::search_bar::*;
use rquote_core::AnimechanQuote;

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
    fn successful_view(
        &self,
        _ctx: &Context<AsyncComponent<Vec<String>, Self>>,
        element: Rc<Vec<String>>,
    ) -> Html {
        html! {
            <SuccessfulComponent list = {element.clone()}/>
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

#[derive(Properties, PartialEq, Clone)]
struct SuccessfulProp {
    pub list: Rc<Vec<String>>,
}

#[function_component(SuccessfulComponent)]
fn successful(props: &SuccessfulProp) -> Html {
    let search_string = use_state(|| String::new());
    let input: Callback<String> = {
        let search_string = search_string.clone();
        move |x: String| search_string.set(x)
    }
    .into();

    let mut vector;

    if search_string.is_empty() {
        vector = props.list.as_ref().clone();
        vector.sort();
    } else {
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
        let mut temp = props
            .list
            .iter()
            .map(|x| (x, matcher.fuzzy_match(x, search_string.as_str())))
            .filter(|(x, score)| { score.map(|x| x > 0).unwrap_or_default() })
            .map(|(x, y)| (x, y.unwrap()))
            .collect::<Vec<_>>(); // Sorting requires memory allocation
        temp.sort_by_key(|(key, value)| -value);
        vector = temp.into_iter().map(|(key, value)| key.clone()).collect();
    }

    let list = vector.into_iter().filter(|x| !x.is_empty()).map(|x| {
        let route = Route::Anime { title: x.clone() };
        html! {
                <Link<Route> to={route} classes={classes!("link-dark")}>
                    {x}
                </Link<Route>>
        }
    });
    html! {
        <>
            <SearchBarComponent {input}/>
            <ListComponent>
                {for list}
            </ListComponent>
        </>
    }
}
