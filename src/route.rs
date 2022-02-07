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
    let links = [
        html! {
            <Link<Route> classes ={classes!("nav-link")} to={Route::AnimeList}>
                { "Anime" }
            </Link<Route>>
        },
        html! {
            <Link<Route> classes ={classes!("nav-link")} to={Route::About}>
                { "About" }
            </Link<Route>>
        },
    ]
        .into_iter()
        .collect::<Html>();
    let navbar_title = "Rquote";

    match route {
        Route::Home => html! {
            <>
                <NavBarComponent title = {navbar_title}>
                    {links}
                </NavBarComponent>
                <Home/>
            </>
        },
        Route::AnimeList => html! {
            <>
                <NavBarComponent title = {navbar_title}>
                    {links}
                </NavBarComponent>
                <AnimeList/>
            </>
        },
        Route::NotFound => html! {
            <>
                <NavBarComponent title = {navbar_title}>
                    {links}
                </NavBarComponent>
                <NotFound/>
            </>
        },
        Route::Anime { title } => {
            let title = urlencoding::decode(title).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent title = {navbar_title} page = {title.clone()}>
                        {links}
                    </NavBarComponent>
                    <Anime title={title.clone()}/>
                </>
            }
        }
        Route::Character { character } => {
            let character = urlencoding::decode(character).unwrap().into_owned();
            html! {
                <>
                    <NavBarComponent title = {navbar_title} page = {character.clone()}>
                        {links}
                    </NavBarComponent>
                    <Character character={character.clone()}/>
                </>
            }
        }
        Route::About => html! {
            <>
                <NavBarComponent title = {navbar_title} page = {"About"}>
                    {links}
                </NavBarComponent>
                <About/>
            </>
        }
    }
}
