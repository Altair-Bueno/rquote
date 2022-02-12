use yew::prelude::*;

use crate::Theme;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProp {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ButtonComponent)]
pub fn button(props: &ButtonProp) -> Html {
    let onclick = props.onclick.clone();
    let theme = use_context::<Theme>().unwrap_or_default();
    let classes = classes!("btn",theme.get_button_class());
    html! {
        <button class = {classes} {onclick}>
            {for props.children.iter()}
        </button>
    }
}