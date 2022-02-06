use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct NavBarProp {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct NavBarComponent;

impl Component for NavBarComponent {
    type Message = ();
    type Properties = NavBarProp;

    fn create(ctx: &Context<Self>) -> Self {
        NavBarComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = &ctx.props().title;
        let nav_items = ctx.props().children
            .iter()
            .map(|x| html! {
                <li class="nav-item">
                    {x}
                </li>
            }).collect::<Html>();
        html! {
            <nav class="navbar sticky-top navbar-expand-lg navbar-light bg-light">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#">{title}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarText" aria-controls="navbarText" aria-expanded="false" aria-label="Toggle navigation">
                      <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarText">
                      <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        {nav_items}
                      </ul>
                    </div>
              </div>
            </nav>
        }
    }
}