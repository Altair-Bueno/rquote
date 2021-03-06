use yew::prelude::*;

use rquote_component::not_found::*;

use crate::route::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <NotFoundComponent<Route> to={Route::Home}/>
    }
}
