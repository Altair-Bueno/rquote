use yew::prelude::*;

use crate::Theme;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProp {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ListComponent)]
pub fn list(props: &ListProp) -> Html {
    let theme: Theme = use_context().unwrap_or_default();
    let child = props.children.iter().enumerate().map(|(id, x)| html! {
            <li class = {classes!("list-group-item",theme.get_background_class())} id = {id.to_string()}>
                {x}
            </li>
        });
    let mut class = props.class.clone();
    class.push("list-group");
    html! {
        <ul {class}>
            {for child}
        </ul>
    }
}
