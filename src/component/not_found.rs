use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NotFoundProp {
    #[prop_or_default]
    pub route: Option<String>,
}

pub struct NotFoundComponent;

impl Component for NotFoundComponent {
    type Message = ();
    type Properties = NotFoundProp;

    fn create(ctx: &Context<Self>) -> Self {
        NotFoundComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="position-absolute top-50 start-50 translate-middle">
                <h1>{"Not Found"}</h1>
                {
                    if let Some(route) = &ctx.props().route {
                        html!{
                            <p>{format!("Couldn't find {route}")}</p>
                        }
                    } else {
                        html!{}
                    }
                }
            </div>
        }
    }
}
