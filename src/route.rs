use yew::prelude::*;
use yew_router::prelude::*;

use about::*;
use anime::*;
use anime_list::*;
use character::*;
// Debug page with all components
#[cfg(debug_assertions)]
use dev::*;
use home::*;
use not_found::*;

use crate::component::nav_bar::*;

mod anime;
mod anime_list;
mod character;
mod home;
mod not_found;
mod about;

#[cfg(debug_assertions)]
mod dev;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/anime")]
    AnimeList,
    #[at("/anime/:title")]
    Anime { title: String },
    #[at("/character/:character")]
    Character { character: String },
    #[at("/about")]
    About,
    #[cfg(debug_assertions)]
    #[cfg_attr(debug_assertions, at("/dev"))]
    Development,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    let links = vec![
        NavBarLink::new("Anime list".to_string(), Route::AnimeList),
        NavBarLink::new("About".to_string(), Route::About),
    ];
    let navbar_props = NavBarProp {
        home: Route::Home,
        title: "Rquote".to_string(),
        link: links,
        active: None,
    };
    let page = match route {
        Route::Home => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <Home/>
            </>
        },
        Route::AnimeList => html! {
            <>
                <NavBarComponent<Route> active = {0} ..navbar_props/>
                <AnimeList/>
            </>
        },
        Route::NotFound => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <NotFound/>
            </>
        },
        Route::Anime { title } => {
            let title = urlencoding::decode(title).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent<Route>  ..navbar_props/>
                    <Anime title={title}/>
                </>
            }
        }
        Route::Character { character } => {
            let character = urlencoding::decode(character).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent<Route> ..navbar_props/>
                    <Character character={character}/>
                </>
            }
        }
        Route::About => html! {
            <>
                <NavBarComponent<Route> active = {1} ..navbar_props/>
                <About/>
            </>
        },
        #[cfg(debug_assertions)]
        Route::Development => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <Dev/>
            </>
        }
    };
    page
}
