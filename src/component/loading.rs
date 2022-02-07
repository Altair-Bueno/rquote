use yew::prelude::*;

#[function_component(LoadingComponent)]
pub fn loading() -> Html {
    html! {
        <div class = "text-center m-1">
            <div class="spinner-border spinner-border-sm ms-auto m-1" role="status" aria-hidden="true"/>
            <strong class= "m-1">{"Loading..."}</strong>
        </div>
    }
}
