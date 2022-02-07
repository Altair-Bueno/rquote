use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::Router;

#[derive(PartialEq, Clone)]
pub struct NavBarLink<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    name: String,
    link: T,
}

impl<T> NavBarLink<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    pub fn new(name: String, link: T) -> NavBarLink<T> {
        NavBarLink {
            name,
            link,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NavBarProp<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    pub home: T,
    pub title: String,
    #[prop_or_default]
    pub link: Vec<NavBarLink<T>>,
    #[prop_or_default]
    pub active: Option<usize>,
}

pub struct NavBarComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    phantom: PhantomData<&'static T>,
}

impl<'a, T> Component for NavBarComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static
{
    type Message = ();
    type Properties = NavBarProp<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        NavBarComponent {
            phantom: Default::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link_list = ctx
            .props()
            .link
            .iter()
            .enumerate()
            .map(|(position, link)| {
                let active = if ctx.props().active.map(|x| x == position).unwrap_or(false) {
                    Some("active")
                } else {
                    None
                };
                let classes = classes!("nav-link","px-3",active);
                html! {
                    <li class = "nav-item" key = {format!("navbar-link-{position}")}>
                        <Link<T> classes ={classes} to={link.link.clone()}>
                            { link.name.as_str() }
                        </Link<T>>
                    </li>
                }
            })
            .collect::<Html>();
        let (context, _) = ctx
            .link()
            .context::<crate::context::Context>(Default::default())
            .expect("Expected context");
        html! {
            <nav class={classes!("navbar", "sticky-top", "navbar-expand-lg",context.theme().get_navbar_class())}>
                <div class="container-fluid">
                    <Link<T> classes ={classes!("navbar-brand", "h1", "mb-0")} to={ctx.props().home.clone()}>
                        { &ctx.props().title }
                    </Link<T>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarText" aria-controls="navbarText" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarText">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                            {link_list}
                        </ul>
                    </div>
              </div>
            </nav>
        }
    }
}
