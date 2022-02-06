use yew::prelude::*;

use crate::component::nav_bar::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <NavBarComponent title="Rquote"/>
    }
}
