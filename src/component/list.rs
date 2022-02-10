use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProp {
    pub children: Children,
}

#[function_component(ListComponent)]
pub fn list(props: &ListProp) -> Html {
    let child = props.children.iter().map(|x| {
        html! {
            <li class = {classes!("list-group-item",)}>
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
