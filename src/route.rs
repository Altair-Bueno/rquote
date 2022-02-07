use std::fmt::Debug;

use yew::prelude::*;
use yew_router::prelude::*;

use about::*;
use anime::*;
use anime_list::*;
use character::*;
use home::*;
use not_found::*;

use crate::component::nav_bar::*;

mod anime;
mod anime_list;
mod character;
mod home;
mod not_found;
mod about;

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
    match route {
        Route::Home => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <Home/>
            </>
        },
        Route::AnimeList => html! {
            <>
                <NavBarComponent<Route> active = {Some(0)} ..navbar_props/>
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
                <NavBarComponent<Route> active = {Some(1)} ..navbar_props/>
                <About/>
            </>
        }
    }
}
