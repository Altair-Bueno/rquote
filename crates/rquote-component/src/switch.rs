use yew::prelude::*;

use crate::Theme;

#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub text: Option<String>,
}

#[function_component(SwitchComponent)]
pub fn loading(props: &SwitchProps) -> Html {
    let theme = use_context::<Theme>().unwrap_or_default();
    let onclick = props.onclick.clone();
    let text = if let Some(text) = &props.text {
        html! {
            <label class={classes!("form-check-label",theme.get_text_class())} for="flexSwitchCheckDefault">
                {text}
            </label>
        }
    } else {
        Html::default()
    };

    html! {
        <div class={classes!("form-check","form-switch")}>
            <input class={classes!("form-check-input")}  type="checkbox" role="switch" id="flexSwitchCheckDefault" {onclick} />
            {text}
        </div>
    }
}
