use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FooterProp {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(FooterComponent)]
pub fn footer(props: &FooterProp) -> Html {
    let _theme = use_context::<crate::context::Theme>().unwrap_or_default();
    html! {
        <footer class={classes!("d-flex","flex-wrap","justify-content-between","align-items-center","py-3","my-4","border-top")}>
        {for props.children.iter()}
        </footer>
    }
}
