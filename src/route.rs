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
use lucky::*;
use not_found::*;
use rquote_component::nav_bar::*;

mod about;
mod anime;
mod anime_list;
mod character;
mod home;
mod not_found;

#[cfg(debug_assertions)]
mod dev;
mod lucky;

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
    #[at("/lucky")]
    Lucky,
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
        NavBarLink::new("I'm feeling lucky".to_string(), Route::Lucky),
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
        },
        Route::Character { character } => {
            let character = urlencoding::decode(character).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent<Route> ..navbar_props/>
                    <Character character={character}/>
                </>
            }
        },
        Route::Lucky => html! {
            <>
                <NavBarComponent<Route> active = {1}..navbar_props/>
                <Lucky/>
            </>
        },
        Route::About => html! {
            <>
                <NavBarComponent<Route> active = {2} ..navbar_props/>
                <About/>
            </>
        },
        #[cfg(debug_assertions)]
        Route::Development => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <Dev/>
            </>
        },
    };
    page
}
