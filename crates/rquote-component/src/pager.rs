use yew::prelude::*;

use crate::Theme;

#[derive(Properties, Clone, PartialEq)]
pub struct PagerProps {
    #[prop_or_default]
    pub next: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub prev: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub page: u32,
}

#[function_component(PagerComponent)]
pub fn pager(props: &PagerProps) -> Html {
    let _theme = use_context::<Theme>().unwrap_or_default();
    let prev = if let Some(onclick) = props.prev.clone() {
        html! {
            <li key = {"prev"} class = {classes!("page-item")}>
            <button class = {classes!("page-link")} {onclick}>{"«"}</button>
            </li>
        }
    } else {
        Html::default()
    };

    let next = if let Some(onclick) = props.next.clone() {
        html! {
            <li key = {"next"} class = {classes!("page-item")}>
                <button class = {classes!("page-link")} {onclick}>{"»"}</button>
            </li>
        }
    } else {
        Html::default()
    };
    let page = html! {
        <li key = {"current"} class = {classes!("page-item")}>
            <span class = {classes!("page-link")}>{props.page}</span>
        </li>
    };
    html! {
        <nav aria-label="Page navigation" class={classes!("align-middle")}>
            <ul class={classes!("pagination","justify-content-center")}>
                {prev}{page}{next}
            </ul>
        </nav>
    }
}
