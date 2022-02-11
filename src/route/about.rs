use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use web_sys::Element;
use yew::prelude::*;

use rquote_component::async_load::{*, ViewAsync};
use rquote_component::Theme;

const README: &str = "https://raw.githubusercontent.com/Altair-Bueno/rquote/master/README.md";
const SMALL_NOTE: &str = "Readme rendered using Rust + WASM ❤️";

#[derive(Debug, Clone, PartialEq)]
struct AboutProvider {
    node_ref: NodeRef,
}

#[async_trait(? Send)]
impl ViewAsync<String> for AboutProvider {
    async fn fetch_data(&self, client: Client) -> Message<String> {
        async fn closure(client: Client) -> Result<String, reqwest::Error> {
            let text = client
                .get(README)
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?;
            let parser = pulldown_cmark::Parser::new(text.as_str());
            let mut out = String::new();
            pulldown_cmark::html::push_html(&mut out, parser);
            Ok(out)
        }
        match closure(client).await {
            Ok(x) => Message::Successful(Rc::new(x)),
            Err(x) => Message::Failed(Rc::new(x)),
        }
    }
    fn successful_view(
        &self,
        element: Rc<String>,
    ) -> Html {
        let theme = use_context::<Theme>().unwrap_or_default();
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(element.as_str());
        let elements = el.get_elements_by_tag_name("a");
        for i in 0..elements.length() {
            elements
                .get_with_index(i)
                .unwrap()
                .set_class_name(theme.get_link_class())
        }
        Html::default()
    }
}

#[function_component(About)]
pub fn about() -> Html {
    let node_ref = Default::default();
    let provider = use_state(move || AboutProvider { node_ref });
    let theme = use_context::<Theme>().unwrap_or_default();

    if let Some(element) = provider.node_ref.cast::<Element>() {
        let element_list = element.get_elements_by_tag_name("a");
        for i in 0..element_list.length() {
            element_list
                .get_with_index(i)
                .unwrap()
                .set_class_name(theme.get_link_class());
        }
    };
    html! {
        <div class = {classes!(theme.get_background_class(),theme.get_text_class(),"shadow-lg", "p-3", "m-3","rounded")}>
            <div ref={provider.node_ref.clone()}/>
            <AsyncComponent<String,AboutProvider> provider = {provider.deref().clone()}/>
            <small>{SMALL_NOTE}</small>
        </div>
    }
}
