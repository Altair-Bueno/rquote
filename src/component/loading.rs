use yew::prelude::*;

#[function_component(LoadingComponent)]
pub fn loading() -> Html {
    let theme = use_context::<crate::context::Theme>()
        .unwrap_or_default();
    html! {
        <div class = {classes!("text-center", "m-1",theme.get_text_class())}>
            <div class={classes!("spinner-border","spinner-border-sm","ms-auto","m-1")} role="status" aria-hidden="true"/>
            <strong class= {classes!("m-1")}>{"Loading..."}</strong>
        </div>
    }
}
