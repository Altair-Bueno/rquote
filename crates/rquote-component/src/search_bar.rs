use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

use crate::Theme;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchBarProps {
    pub input: Callback<String>,
    #[prop_or("Search".to_string())]
    pub placeholder: String,
    #[prop_or_default]
    pub class: Classes,
}

// https://github.com/yewstack/yew/blob/master/examples/password_strength/src/text_input.rs
fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap();
    let event_target = event.target().unwrap();
    let target: HtmlInputElement = event_target.dyn_into().unwrap();
    target.value()
}

/// A Bootstrap searchbar
///
/// ```rust
/// use yew::prelude::*;
/// use rquote_component::search_bar::*;
///
/// #[function_component(App)]
/// fn app()->Html {
///     let class = classes!("bg-dark");
///     let placeholder = "Search something".to_string();
///     let input = Callback::from(|x:String|web_sys::console::log_1(&x.into()));
///     html!{
///         <SearchBarComponent {class} {placeholder} {input}/>
///     }
/// }
/// ```
#[function_component(SearchBarComponent)]
pub fn search(props: &SearchBarProps) -> Html {
    let theme: Theme = use_context().unwrap_or_default();
    let placeholder = props.placeholder.clone();
    let callback = props.input.clone();
    let oninput = move |x: InputEvent| callback.emit(get_value_from_input_event(x));
    let onkeydown = |x: KeyboardEvent| {
        if x.key_code() == 13 {
            x.prevent_default()
        }
    };
    let mut class = props.class.clone();
    class.push("form-control");
    class.push(theme.get_background_class());
    class.push(theme.get_text_class());
    html! {
        <form class={classes!("d-flex")}>
            <input
                {class}
                type="search"
                role="searchbox"
                aria-label="Search"
                placeholder={placeholder}
                label="Search"
                {oninput}
                {onkeydown}/>
        </form>
    }
}
