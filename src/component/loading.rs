use yew::prelude::*;

pub struct Loading;

impl Component for Loading {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Loading
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class = "text-center m-1">
                <div class="spinner-border spinner-border-sm ms-auto m-1" role="status" aria-hidden="true"/>
                <strong class= "m-1">{"Loading..."}</strong>
            </div>
        }
    }
}
