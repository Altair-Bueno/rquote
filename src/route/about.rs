use std::error::Error;
use std::ops::Deref;
use std::rc::Rc;

use reqwest::Client;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};

use rquote_component::error::*;
use rquote_component::loading::LoadingComponent;
use rquote_component::Theme;
use rquote_core::wrapper::ClientWrapper;

const README: &str = "https://raw.githubusercontent.com/Altair-Bueno/rquote/master/README.md";
const SMALL_NOTE: &str = "Readme rendered using Rust + WASM ❤️";

async fn fetch_data(client:&Client) -> Result<String,Rc<dyn Error>> {
    async fn closure(client: &Client) -> Result<String, reqwest::Error> {
        let text = client.get(README)
            .send().await?
            .error_for_status()?
            .text().await?;
        let parser = pulldown_cmark::Parser::new(text.as_str());
        let mut out = String::new();
        pulldown_cmark::html::push_html(&mut out, parser);
        Ok(out)
    }
    match closure(client).await {
        Ok(data)=> Ok(data),
        Err(error) => {
            let into_trait: Rc<dyn Error> = Rc::new(error);
            Err(into_trait)
        }
    }
}
#[function_component(About)]
pub fn about() -> Html {
    let node_ref = use_state(|| NodeRef::default());
    let client = use_context::<ClientWrapper>().unwrap_or_default();
    let theme = use_context::<Theme>().unwrap_or_default();

    let state = use_async(async move {fetch_data(client.as_ref()).await});
    {
        let state = state.clone();
        use_effect_once(move||{
            state.run();
            ||()
        })
    }
    {
        let node_ref = node_ref.clone();
        let theme = theme.clone();
        let state = state.clone();
        use_effect(move ||{
            if let Some(element) = node_ref.cast::<Element>() {
                if let Some(data) = &state.data {
                    element.set_inner_html(data);
                }
                let element_list = element.get_elements_by_tag_name("a");
                for i in 0..element_list.length() {
                    element_list
                        .get_with_index(i)
                        .unwrap()
                        .set_class_name(theme.get_link_class());
                }
            };
            ||()
        });
    }

    if state.loading {
        html!{<LoadingComponent/>}
    } else if let Some(data) = &state.data {
        html! {
            <div class = {classes!(theme.get_background_class(),theme.get_text_class(),"shadow-lg", "p-3", "m-3","rounded")}>
                <div ref={node_ref.deref().clone()}/>
                <small>{SMALL_NOTE}</small>
            </div>
        }
    } else if let Some(error) = &state.error {
        let error = error.clone();
        let severity = Severity::Danger;
        html!{<ErrorComponent {severity} {error}/>}
    } else {Default::default()}
}
