use std::rc::Rc;

use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use yew::prelude::*;
use yew_router::prelude::*;

use rquote_component::async_load::*;
use rquote_component::async_load::ViewAsync;
use rquote_component::list::*;
use rquote_component::search_bar::*;
use rquote_component::Theme;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::route::Route;

#[derive(Debug, PartialEq, Clone)]
pub struct AnimeListProvider {
    client: ClientWrapper,
}

#[async_trait(? Send)]
impl ViewAsync<Vec<String>> for AnimeListProvider {
    async fn fetch_data(&self) -> Message<Vec<String>> {
        let response = AnimechanQuote::get_anime_list(self.client.as_ref()).await;
        match response {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(err) => Message::Failed(Rc::new(err)),
        }
    }
    fn successful_view(
        &self,
        element: Rc<Vec<String>>,
    ) -> Html {
        html! {
            <SuccessfulComponent list = {element.clone()}/>
        }
    }
}

#[function_component(AnimeList)]
pub fn anime_list() -> Html {
    let provider = AnimeListProvider {
        client: use_context().unwrap_or_default(),
    };
    html! {
        <AsyncComponent<Vec<String>,AnimeListProvider> {provider}/>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct SuccessfulProp {
    pub list: Rc<Vec<String>>,
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
        vector = props.list.as_ref().clone();
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
        html! {
                <Link<Route> to={route} classes={classes!(theme.get_link_class())}>
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
