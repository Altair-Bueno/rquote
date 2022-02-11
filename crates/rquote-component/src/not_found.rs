use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Theme;

#[derive(Properties, PartialEq, Clone)]
pub struct NotFoundProp<T>
where
    T: Clone + Routable + PartialEq + 'static,
{
    pub to: T,
    #[prop_or_else(|| "Page Not Found".to_string())]
    pub title: String,
    #[prop_or_else(|| "We couldn't find the page you requested".to_string())]
    pub message: String,
    #[prop_or_else(|| "Go home".to_string())]
    pub info: String,
}


#[function_component(NotFoundComponent)]
pub fn not_found<T>(props: &NotFoundProp<T>) -> Html
    where
        T: Clone + Routable + PartialEq + 'static,
{
    let theme = use_context::<Theme>().unwrap_or_default();
    let text_class = theme.get_text_class();
    let div_class = classes!(
        text_class,
        theme.get_background_class(),
        // Center
        "position-absolute",
        "top-50",
        "start-50",
        "translate-middle",
        // Shadow
        "shadow-lg",
        "p-3",
        "mb-3",
        "rounded"
    );
    html! {
        <div class={div_class}>
            <h1>{&props.title}</h1>
            <p>{&props.message}</p>
            <Link<T> to = {props.to.clone()} classes={classes!(theme.get_link_class())}>
                <strong>{&props.info}</strong>
            </Link<T>>
        </div>
    }
}
