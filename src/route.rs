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


/// Different available routes for Rquote
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    /// Homepage
    #[at("/")]
    Home,
    /// Anime list
    #[at("/anime")]
    AnimeList,
    #[at("/anime/:title")]
    /// All quotes for an anime
    Anime { title: String },
    /// All quotes for a character
    #[at("/character/:character")]
    Character { character: String },
    /// About section
    #[at("/about")]
    About,
    /// I'm feeling lucky
    #[at("/lucky")]
    Lucky,
    /// Test page
    #[cfg(debug_assertions)]
    #[cfg_attr(debug_assertions, at("/dev"))]
    Development,
    /// Not found page
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
        image: Some("resources/logo.png".to_string()),
        link: links,
        active: None,
    };
    let page = match route {
        Route::Home => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <main>
                    <Home/>
                </main>
            </>
        },
        Route::AnimeList => html! {
            <>
                <NavBarComponent<Route> active = {0} ..navbar_props/>
                <main>
                    <AnimeList/>
                </main>
            </>
        },
        Route::NotFound => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <main>
                    <NotFound/>
                </main>
            </>
        },
        Route::Anime { title } => {
            let title = urlencoding::decode(title).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent<Route>  ..navbar_props/>
                    <main>
                        <Anime title={title}/>
                    </main>
                </>
            }
        },
        Route::Character { character } => {
            let character = urlencoding::decode(character).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent<Route> ..navbar_props/>
                    <main>
                        <Character character={character}/>
                    </main>
                </>
            }
        },
        Route::Lucky => html! {
            <>
                <NavBarComponent<Route> active = {1}..navbar_props/>
                <main>
                    <Lucky/>
                </main>
            </>
        },
        Route::About => html! {
            <>
                <NavBarComponent<Route> active = {2} ..navbar_props/>
                <main>
                    <About/>
                </main>
            </>
        },
        #[cfg(debug_assertions)]
        Route::Development => html! {
            <>
                <NavBarComponent<Route> ..navbar_props/>
                <main>
                    <Dev/>
                </main>
            </>
        },
    };
    page
}
