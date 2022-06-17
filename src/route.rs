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
    let (active, inner) = match route {
        // Development pages
        #[cfg(debug_assertions)]
        Route::Development => (None, html! {<Dev/>}),
        // Main pages
        Route::Home => (None, html! {<Home/>}),
        Route::AnimeList => (Some(0), html! {<AnimeList/>}),
        Route::Lucky => (Some(1), html! {<Lucky/>}),
        Route::About => (Some(2), html! {<About/>}),

        Route::Anime { title } => {
            let title = urlencoding::decode(title).unwrap().into_owned();
            (None, html! {<Anime title={title}/>})
        }
        Route::Character { character } => {
            let character = urlencoding::decode(character).unwrap().into_owned();
            (None, html! {<Character character={character}/>})
        }
        Route::NotFound => (None, html! { <NotFound /> }),
    };
    let navbar_props = NavBarProp {
        home: Route::Home,
        title: "Rquote".to_string(),
        image: Some("resources/logo.png".to_string()),
        link: links,
        active,
    };
    html! {
        <>
            <NavBarComponent<Route> ..navbar_props/>
            <main>
                {inner}
            </main>
        </>
    }
}
