use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct NotFoundProp {
    #[prop_or_default]
    pub message: Option<String>,
}

#[function_component(NotFoundComponent)]
pub fn not_found(props: &NotFoundProp) -> Html {
    let head = "Page Not Found";
    let message = html! {
        <p>{
            props
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
