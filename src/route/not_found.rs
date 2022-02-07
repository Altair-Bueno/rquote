use yew::prelude::*;

use crate::component::nav_bar::*;
use crate::component::not_found::*;
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <NotFoundComponent<Route> home={Some(Route::Home)}/>
    }
}
