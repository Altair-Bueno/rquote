use yew::prelude::*;
use yew_router::prelude::*;

use anime::*;
use anime_list::*;
use character::*;
use home::*;
use not_found::*;

use crate::component::nav_bar::*;

mod home;
mod anime;
mod anime_list;
mod character;
mod not_found;

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
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    let inner = match route {
        Route::Home => html! {<Home/>},
        Route::Anime { title } => html! {<Anime title={title.clone()}/>},
        Route::AnimeList => html! {<AnimeList/>},
        Route::Character { character } => html! {<Character character={character.clone()}/>},
        Route::NotFound => html! {<NotFound/>},
    };
    html! {
        <>
            <NavBarComponent title = "Rquote">
                <Link<Route> classes ={classes!("nav-link")} to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> classes ={classes!("nav-link")} to={Route::AnimeList}>{ "Anime" }</Link<Route>>
            </NavBarComponent>
            {inner}
        </>
    }
}
