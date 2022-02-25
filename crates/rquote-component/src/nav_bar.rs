use yew::prelude::*;
use yew_router::prelude::*;

use crate::Theme;

/// Describes a navigation bar link for a navbar
#[derive(PartialEq, Clone)]
pub struct NavBarLink<Router>
    where
        Router: Clone + Routable + PartialEq,
{
    name: String,
    link: Router,
}

impl<Router> NavBarLink<Router>
    where
        Router: Clone + Routable + PartialEq,
{
    /// Creates a new navbar link
    pub fn new(name: String, link: Router) -> NavBarLink<Router> {
        NavBarLink { name, link }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NavBarProp<Router>
    where
        Router: Clone + Routable + PartialEq,
{
    pub home: Router,
    pub title: String,
    #[prop_or_default]
    pub link: Vec<NavBarLink<Router>>,
    #[prop_or_default]
    pub active: Option<usize>,
}


/// A Bootstrap navbar
///
/// ```rust
/// use yew::prelude::*;
/// use yew_router::prelude::*;
/// use rquote_component::nav_bar::*;
///
/// #[derive(Routable, Clone, PartialEq)]
/// enum Router {
///     #[at("/")]
///     Home,
///     #[at("/foo")]
///     Foo,
/// }
///
/// #[function_component(App)]
/// fn app()->Html {
///     let home = Router::Home;
///     let title = "Homepage".to_string();
///     let link = vec![
///         NavBarLink::new("Foo".to_string(),Router::Foo),
///     ];
///     let active = None;
///     html!{
///         <NavBarComponent<Router> {home} {title} {link} {active} />
///     }
/// }
/// ```
#[function_component(NavBarComponent)]
pub fn navbar<Router>(props: &NavBarProp<Router>) -> Html
    where
        Router: Clone + Routable + PartialEq + 'static,
{
    let link_list = props
        .link
        .iter()
        .enumerate()
        .map(|(position, link)| {
            let active = if props.active.map(|x| x == position).unwrap_or(false) {
                Some("active")
            } else {
                None
            };
            let classes = classes!("nav-link", "px-3", active);
            html! {
                <li class = {classes!("nav-item")} key = {format!("navbar-link-{position}")}>
                    <Link<Router> classes ={classes} to={link.link.clone()}>
                        { link.name.as_str() }
                    </Link<Router>>
                </li>
            }
        })
        .collect::<Html>();
    let theme = use_context::<Theme>().unwrap_or_default();
    html! {
        <nav class={classes!("navbar", "sticky-top", "navbar-expand-lg",theme.get_navbar_class(),theme.get_background_class())}>
            <div class={classes!("container-fluid")}>
                <Link<Router> classes ={classes!("navbar-brand", "h1", "mb-0")} to={props.home.clone()}>
                    { &props.title }
                </Link<Router>>
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

