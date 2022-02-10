use yew::prelude::*;

use rquote_component::pager::*;

#[function_component(Dev)]
pub fn dev() -> Html {
    let page = 10;
    let prev: Callback<MouseEvent> = { |_| () }.into();
    let next = prev.clone();
    html! {
        <PagerComponent {page} {prev} {next}/>
    }
}
