use yew::prelude::*;

use rquote_component::button::*;

#[function_component(Dev)]
pub fn dev() -> Html {
    let prev: Callback<MouseEvent> = { |_| web_sys::console::log_1(&"Something".into()) }.into();
    html! {
        <ButtonComponent onclick = {prev}>
        {"Hello world"}
        </ButtonComponent>
    }
}
