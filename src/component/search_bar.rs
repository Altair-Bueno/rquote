use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchBarProps {
    pub input: Callback<String>,
    #[prop_or("Search".to_string())]
    pub placeholder: String,
}

// https://github.com/yewstack/yew/blob/master/examples/password_strength/src/text_input.rs
fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap();
    let event_target = event.target().unwrap();
    let target: HtmlInputElement = event_target.dyn_into().unwrap();
    target.value()
}

#[function_component(SearchBarComponent)]
pub fn search(props: &SearchBarProps) -> Html {
    let placeholder = props.placeholder.clone();
    let callback = props.input.clone();
    let oninput = move |x: InputEvent| callback.emit(get_value_from_input_event(x));
    let onkeydown = |x: KeyboardEvent| if x.key_code() == 13 { x.prevent_default() };
    html! {
        <form class="d-flex">
            <input
                class={classes!("form-control","m-3")}
                type="search"
                aria-label="Search"
                placeholder={placeholder}
                {oninput}
                {onkeydown}/>
        </form>
    }
}
