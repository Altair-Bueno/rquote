use yew::prelude::*;

use crate::Theme;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProp {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

/// A Bootstrap button
///
/// ```rust
/// use yew::prelude::*;
/// use rquote_component::button::*;
///
/// #[function_component(App)]
/// fn app()->Html {
///     let onclick = Callback::from(|_| web_sys::console::log_1(&"Hello world".into()));
///     let class = classes!("bg-primary");
///     html!{
///         <ButtonComponent {onclick} {class}>
///             {"This is a button"}
///         </ButtonComponent>
///     }
/// }
/// ```
#[function_component(ButtonComponent)]
pub fn button(props: &ButtonProp) -> Html {
    let onclick = props.onclick.clone();
    let theme = use_context::<Theme>().unwrap_or_default();
    let mut classes = props.class.clone();
    classes.push("btn");
    classes.push(theme.get_button_class());
    html! {
        <button class = {classes} {onclick}>
            {for props.children.iter()}
        </button>
    }
}