use yew::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::loading::*;
use crate::component::not_found::*;
use crate::component::quote::*;
use crate::route::Route;

#[function_component(Dev)]
pub fn dev() -> Html {
    html! {
        <>
        <QuoteComponent quote = {AnimechanQuote::get_example()}/>
        <br/>
        <LoadingComponent/>
        </>
    }
    ;
    html! {
        <>
        <NotFoundComponent<Route> to ={Route::Home} />
        </>
    }
}