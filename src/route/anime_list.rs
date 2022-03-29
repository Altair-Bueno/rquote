use std::error::Error;
use std::rc::Rc;

use fuzzy_matcher::FuzzyMatcher;
use reqwest::Client;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::*;

use rquote_component::error::*;
use rquote_component::list::*;
use rquote_component::loading::LoadingComponent;
use rquote_component::search_bar::*;
use rquote_component::Theme;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::route::Route;

async fn fetch_data(client: &Client) -> Result<Vec<String>, Rc<dyn Error>> {
    let response = AnimechanQuote::get_anime_list(client).await;
    match response {
        Ok(quote) => Ok(quote),
        Err(error) => {
            let into_trait: Rc<dyn Error> = Rc::new(error);
            Err(into_trait)
        }
    }
}

#[function_component(AnimeList)]
pub fn anime_list() -> Html {
    let client: ClientWrapper = use_context().unwrap_or_default();
    let state = use_async_with_options(
        async move { fetch_data(client.as_ref()).await },
        UseAsyncOptions::enable_auto()
    );

    if let Some(list) = &state.data {
        let list = list.clone();
        html! {
            <SuccessfulComponent {list}/>
        }
    } else if let Some(error) = &state.error {
        let severity = Severity::Danger;
        let error = error.clone();
        html! {<ErrorComponent {severity} {error}/>}
    } else { html! {<LoadingComponent/>} }
}

#[derive(Properties, PartialEq, Clone)]
struct SuccessfulProp {
    pub list: Vec<String>,
}

#[function_component(SuccessfulComponent)]
fn successful(props: &SuccessfulProp) -> Html {
    let theme: Theme = use_context().unwrap_or_default();
    let search_string = use_state(|| String::new());
    let input: Callback<String> = {
        let search_string = search_string.clone();
        move |x: String| search_string.set(x)
    }
        .into();

    let mut vector;

    if search_string.is_empty() {
        vector = props.list.clone();
        vector.sort();
    } else {
        let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
        let mut temp = props
            .list
            .iter()
            .map(|x| (x, matcher.fuzzy_match(x, search_string.as_str())))
            .filter(|(_, score)| { score.map(|x| x > 0).unwrap_or_default() })
            .map(|(string, score)| (string, score.unwrap()))
            .collect::<Vec<_>>(); // Sorting requires memory allocation
        temp.sort_unstable_by_key(|(_, score)| -score);
        vector = temp.into_iter().map(|(string, _)| string.clone()).collect();
    }

    let list = vector.into_iter().filter(|x| !x.is_empty()).map(|x| {
        let route = Route::Anime { title: urlencoding::encode(&x).to_string() };
        let title = x.to_string();
        let theme = theme.clone();
        AnimeTitleWrapper {
            title,
            route,
            theme,
        }
    }).collect::<Vec<_>>();
    html! {
        <div class = {classes!("container")}>
            <SearchBarComponent {input} class = {classes!("m-3")}/>
            <ListComponent<AnimeTitleWrapper> class = {classes!("mx-3")} children = {list}/>
        </div>
    }
}

#[derive(PartialEq)]
struct AnimeTitleWrapper {
    title: String,
    route: Route,
    theme: Theme,
}

impl ListElement for AnimeTitleWrapper {
    fn key(&self) -> String {
        self.title.to_string()
    }

    fn view(&self) -> Html {
        let route = self.route.clone();
        let title = self.title.clone();
        html! {
            <Link<Route> to={route} classes={classes!(self.theme.get_link_class())}>
                    {title}
            </Link<Route>>
        }
    }
}