use yew::prelude::*;

use crate::component::search_bar::*;

#[function_component(Dev)]
pub fn dev() -> Html {
    let closure = |x: String| web_sys::console::log_1(&x.into());
    let closure: Callback<String> = closure.into();
    html! {
        <SearchBarComponent input = {closure}/>
    }
}