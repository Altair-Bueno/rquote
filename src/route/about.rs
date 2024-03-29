use std::error::Error;
use std::ops::Deref;
use std::rc::Rc;

use gloo_net::http::Request;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use rquote_component::error::*;
use rquote_component::loading::LoadingComponent;
use rquote_component::Theme;

const README: &str = "https://api.github.com/repos/Altair-Bueno/rquote/contents/README.md";
const ACCEPT_RAW_HEADER: &str = "application/vnd.github.VERSION.raw";
const SMALL_NOTE: &str = "Readme rendered using Rust + WASM ❤️";

async fn fetch_data() -> Result<String, Rc<dyn Error>> {
    async fn closure() -> Result<String, gloo_net::Error> {
        let text = Request::get(README)
            .header("Accept", ACCEPT_RAW_HEADER)
            .send()
            .await?
            .text()
            .await?;
        let parser = pulldown_cmark::Parser::new(text.as_str());
        let mut out = String::new();
        pulldown_cmark::html::push_html(&mut out, parser);
        Ok(out)
    }
    match closure().await {
        Ok(data) => Ok(data),
        Err(error) => {
            let into_trait: Rc<dyn Error> = Rc::new(error);
            Err(into_trait)
        }
    }
}
#[function_component(About)]
pub fn about() -> Html {
    let node_ref = use_state(|| NodeRef::default());
    let theme = use_context::<Theme>().unwrap_or_default();

    let state = use_async_with_options(
        async move { fetch_data().await },
        UseAsyncOptions::enable_auto(),
    );
    {
        let node_ref = node_ref.clone();
        let theme = theme.clone();
        let state = state.clone();
        use_effect(move || {
            if let Some(element) = node_ref.cast::<Element>() {
                if let Some(data) = &state.data {
                    element.set_inner_html(data);
                    // Add the corresponding theme class to each <a> tag
                    let element_list = element.get_elements_by_tag_name("a");
                    for i in 0..element_list.length() {
                        element_list
                            .get_with_index(i)
                            .unwrap()
                            .set_class_name(theme.get_link_class());
                    }
                }
            };
            || ()
        });
    }

    if let Some(_) = &state.data {
        html! {
            <div class = {classes!(theme.get_background_class(),theme.get_text_class(),"shadow-lg","rounded","container")}>
                <div class = {classes!("col","p-2","m-2")}>
                    <div ref={node_ref.deref().clone()}/>
                    <small>{SMALL_NOTE}</small>
                </div>
            </div>
        }
    } else if let Some(error) = &state.error {
        let error = error.clone();
        let severity = Severity::Danger;
        html! {<ErrorComponent {severity} {error}/>}
    } else {
        html! {<LoadingComponent/>}
    }
}
