use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct NotFoundProp {
    #[prop_or_default]
    pub message: Option<String>,
}

pub struct NotFoundComponent;

impl Component for NotFoundComponent {
    type Message = ();
    type Properties = NotFoundProp;

    fn create(ctx: &Context<Self>) -> Self {
        NotFoundComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let head = "Page Not Found";
        let message = html! {
            <p>{
                ctx
                    .props()
                    .message
                    .as_ref()
                    .map(|x| x.as_str())
                    .unwrap_or("We could't find the page you requested")
                }
            </p>
        };
        let help = html! {
            <Link<Route> to = {Route::Home}>{format!("Return to home")}</Link<Route>>
        };
        html! {
            <div class="position-absolute top-50 start-50 translate-middle">
                <h1>{head}</h1>
                {message}
                {help}
            </div>
        }
    }
}
