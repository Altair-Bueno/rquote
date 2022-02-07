use yew::prelude::*;

use crate::AnimechanQuote;
use crate::component::loading::*;
use crate::component::quote::*;

#[function_component(Dev)]
pub fn dev() -> Html {
    html! {
        <>
        <QuoteComponent quote = {Box::new(AnimechanQuote::get_example())}/>
        <QuoteComponent quote = {Box::new(AnimechanQuote::get_example())}/>
        <QuoteComponent quote = {Box::new(AnimechanQuote::get_example())}/>
        <QuoteComponent quote = {Box::new(AnimechanQuote::get_example())}/>
        <br/>
        <LoadingComponent/>
        </>
    }
}