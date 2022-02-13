use yew::prelude::*;

use crate::Theme;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProp {
    pub children: Children,
}

#[function_component(ListComponent)]
pub fn list(props: &ListProp) -> Html {
    let theme: Theme = use_context().unwrap_or_default();
    let child = props.children.iter().map(|x| {
        html! {
            <li class = {classes!("list-group-item",theme.get_background_class())}>
                {x}
            </li>
        }
    });
    html! {
        <ul class = {classes!("list-group","m-3")}>
            {for child}
        </ul>
    }
}
