use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone)]
pub struct NavBarLink<T>
where
    T: Clone + Routable + PartialEq,
{
    name: String,
    link: T,
}

impl<T> NavBarLink<T>
    where
        T: Clone + Routable + PartialEq,
{
    pub fn new(name: String, link: T) -> NavBarLink<T> {
        NavBarLink { name, link }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NavBarProp<T>
    where
        T: Clone + Routable + PartialEq,
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
        T: Clone + Routable + PartialEq,
{
    phantom: PhantomData<T>,
}

impl<'a, T> Component for NavBarComponent<T>
    where
        T: Clone + Routable + PartialEq + 'static,
{
    type Message = ();
    type Properties = NavBarProp<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        NavBarComponent {
            phantom: Default::default(),
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
                let classes = classes!("nav-link", "px-3", active);
                html! {
                    <li class = {classes!("nav-item")} key = {format!("navbar-link-{position}")}>
                        <Link<T> classes ={classes} to={link.link.clone()}>
                            { link.name.as_str() }
                        </Link<T>>
                    </li>
                }
            })
            .collect::<Html>();
        let theme = ctx
            .link()
            .context::<crate::context::Theme>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default();
        html! {
            <nav class={classes!("navbar", "sticky-top", "navbar-expand-lg",theme.get_navbar_class(),theme.get_background_class())}>
                <div class={classes!("container-fluid")}>
                    <Link<T> classes ={classes!("navbar-brand", "h1", "mb-0")} to={ctx.props().home.clone()}>
                        { &ctx.props().title }
                    </Link<T>>
                    <button class={classes!("navbar-toggler")} type="button" data-bs-toggle="collapse" data-bs-target="#navbarText" aria-controls="navbarText" aria-expanded="false" aria-label="Toggle navigation">
                        <span class={classes!("navbar-toggler-icon")}></span>
                    </button>
                    <div class={classes!("collapse","navbar-collapse")} id="navbarText">
                        <ul class={classes!("navbar-nav","me-auto","mb-2","mb-lg-0")}>
                            {link_list}
                        </ul>
                    </div>
              </div>
            </nav>
        }
    }
}
