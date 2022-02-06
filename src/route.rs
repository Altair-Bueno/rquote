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
        Route::AnimeList => html! {<AnimeList/>},
        Route::NotFound => html! {<NotFound/>},
        Route::Anime { title } => {
            let title = urlencoding::decode(title).unwrap().into_owned();
            html! {<Anime title={title}/>}
        }
        Route::Character { character } => {
            let character = urlencoding::decode(character).unwrap().into_owned();
            html! {<Character character={character}/>}
        }
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
