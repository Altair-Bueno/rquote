use yew::prelude::*;

use crate::Theme;

#[derive(Properties, Clone, PartialEq)]
pub struct ListProp {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// Creates a Bootstrap unordered list
///
/// ```rust
/// use yew::prelude::*;
/// use rquote_component::list::*;
///
/// #[function_component(App)]
/// fn app()->Html {
///     let class = classes!("bg-dark");
///     html!{
///         <ListComponent {class}>
///             {"First child"}
///             {"Second child"}
///         </ListComponent>
///     }
/// }
/// ```
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
    let mut class = props.class.clone();
    class.push("list-group");
    html! {
        <ul {class}>
            {for child}
        </ul>
    }
}
