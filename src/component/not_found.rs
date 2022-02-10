use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

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

pub struct NotFoundComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static,
{
    phantom: PhantomData<&'static T>,
}

impl<T> Component for NotFoundComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static,
{
    type Message = ();
    type Properties = NotFoundProp<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        NotFoundComponent {
            phantom: Default::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = ctx
            .link()
            .context::<crate::context::Theme>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default();
        let props = ctx.props();
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
}
