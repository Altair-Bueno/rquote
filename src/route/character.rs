use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use reqwest::Client;
use yew::prelude::*;
use yew_hooks::{use_async, use_search_param};
use yew_router::prelude::*;

use rquote_component::error::*;
use rquote_component::loading::*;
use rquote_component::pager::*;
use rquote_component::Theme;
use rquote_core::AnimechanQuote;
use rquote_core::wrapper::ClientWrapper;

use crate::custom::quote::*;
use crate::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterProp {
    pub character: String,
}

async fn fetch_data(client: &Client, character: &str, page: Option<u32>) -> Result<Vec<AnimechanQuote>, Rc<dyn Error>> {
    let response = AnimechanQuote::get_quote_character(client, character, page).await;
    match response {
        Ok(quote) => Ok(quote),
        Err(error) => {
            let into_trait: Rc<dyn Error> = Rc::new(error);
            Err(into_trait)
        }
    }
}

#[function_component(Character)]
pub fn character(props: &CharacterProp) -> Html {
    let theme = use_context::<Theme>().unwrap_or_default();
    let client = use_context::<ClientWrapper>().unwrap_or_default();
    let title = props.character.as_str();
    let init = use_search_param("page".to_string()).unwrap_or_default().parse::<u32>().unwrap_or_default();
    let page = use_state(|| init);

    let next = {
        let page = page.clone();
        Some(Callback::from(move |_: MouseEvent| page.set(*page + 1)))
    };
    let prev = if *page == 0 { None } else {
        let page = page.clone();
        Some(Callback::from(move |_: MouseEvent| page.set(*page - 1)))
    };

    let fetcher = {
        let page = page.clone();
        let character = props.character.clone();
        use_async(async move { fetch_data(client.as_ref(), character.as_str(), Some(*page)).await })
    };

    {
        let page = page.clone();
        let fetcher = fetcher.clone();
        let character = title.to_string();
        let history = use_history().unwrap();
        use_effect_with_deps(move |page| {
            history.push_with_query(
                Route::Character { character },
                HashMap::from([("page", **page)]),
            ).unwrap();
            fetcher.run();
            || ()
        }, page)
    }

    let content = if fetcher.loading {
        html! {<LoadingComponent/>}
    } else if let Some(quote_list) = &fetcher.data {
        quote_list.iter().map(|quote| html! {
            <div class = "m-2">
                <QuoteComponent quote = {quote.clone()} footer = {false}/>
            </div>
        }).collect()
    } else if let Some(error) = &fetcher.error {
        let severity = Severity::Danger;
        let error = error.clone();
        html! {<ErrorComponent {severity} {error}/>}
    } else { Default::default() };

    html! {
        <div class={classes!("container")}>
            <h1 class = {classes!("ms-3","my-3",theme.get_text_class())}>
                    {title}
                    <small class = {classes!(theme.get_secondary_text_class())}>{" - Character"}</small>
            </h1>
            {content}
            <PagerComponent page = {*page} {next} {prev}/>
        </div>
    }
}
