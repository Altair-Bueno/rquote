use yew::prelude::*;

use crate::animechan::AnimechanQuote;
use crate::component::loading::*;
use crate::component::quote::*;

#[function_component(Dev)]
pub fn dev() -> Html {
    html! {
        <>
        <QuoteComponent quote = {AnimechanQuote::get_example()}/>
        <br/>
        <LoadingComponent/>
        </>
    }
}