use std::rc::Rc;

use async_trait::async_trait;
use reqwest::Client;
use web_sys::Element;
use yew::prelude::*;

use rquote_component::async_load::{ViewAsync, *};
use rquote_component::Theme;

const README: &str = "https://raw.githubusercontent.com/Altair-Bueno/rquote/master/README.md";
const SMALL_NOTE: &str = "Readme rendered using Rust + WASM ❤️";

#[async_trait(? Send)]
impl ViewAsync<String> for About {
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
        ctx: &Context<AsyncComponent<String, Self>>,
        element: Rc<String>,
    ) -> Html {
        let theme = ctx
            .link()
            .context::<Theme>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default();
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

#[derive(Debug, Clone, PartialEq)]
pub struct About {
    node_ref: NodeRef,
}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        About {
            node_ref: Default::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = ctx
            .link()
            .context::<Theme>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default();
        html! {
            <div class = {classes!(theme.get_background_class(),theme.get_text_class(),"shadow-lg", "p-3", "m-3","rounded")}>
                <div ref={self.node_ref.clone()}/>
                <AsyncComponent<String,Self> provider = {self.clone()}/>
                <small>{SMALL_NOTE}</small>
            </div>
        }
    }
}
