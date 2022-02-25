use yew::prelude::*;

use crate::Theme;

pub trait ListElement {
    fn key(&self) -> String;
    fn view(&self) -> Html;
}

#[derive(Properties, Clone, PartialEq)]
pub struct ListProp<LISTELEMENT>
    where
        LISTELEMENT: ListElement + std::cmp::PartialEq
{
    #[prop_or_default]
    pub children: Vec<LISTELEMENT>,
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
pub fn list<LISTELEMENT>(props: &ListProp<LISTELEMENT>) -> Html
    where
        LISTELEMENT: ListElement + std::cmp::PartialEq
{
    let theme: Theme = use_context().unwrap_or_default();
    let child = props.children.iter().map(|x| html! {
            <li class = {classes!("list-group-item",theme.get_background_class())} id = {x.key()}>
                {x.view()}
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
