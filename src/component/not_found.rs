use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct NotFoundProp<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    #[prop_or_default]
    pub message: Option<String>,
    #[prop_or_default]
    pub home: Option<T>,
}

pub struct NotFoundComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    phantom: PhantomData<&'static T>,
}

impl<T> Component for NotFoundComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    type Message = ();
    type Properties = NotFoundProp<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        NotFoundComponent {
            phantom: Default::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = use_context::<crate::context::Context>()
            .expect("Expected context");
        let props = ctx.props();
        let head = "Page Not Found";
        let message = html! {
            <p>{
                props
                    .message
                    .as_ref()
                    .map(|x| x.as_str())
                    .unwrap_or("We couldn't find the page you requested")
                }
            </p>
        };
        let help = if let Some(home) = &props.home {
            html! {
                <Link<T> to = {home.clone()} classes={classes!(context.theme().get_link_class())}>
                    {format!("Return to home")}
                </Link<T>>
            }
        } else {
            Html::default()
        };
        let help = html! {

        };
        html! {
            <div class={classes!("position-absolute","top-50","start-50","translate-middle")}>
                <h1>{head}</h1>
                {message}
                {help}
            </div>
        }
    }
}
