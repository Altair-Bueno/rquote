use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

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
        let theme = ctx.link().context::<crate::context::Theme>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default();
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
                <Link<T> to = {home.clone()} classes={classes!(theme.get_link_class())}>
                    {format!("Return to home")}
                </Link<T>>
            }
        } else {
            Html::default()
        };
        let text_class = theme.get_text_class();
        html! {
            <div class={classes!("justify-content-center"," align-items-center",text_class)}>
                <h1>{head}</h1>
                {message}
                {help}
            </div>
        }
    }
}
