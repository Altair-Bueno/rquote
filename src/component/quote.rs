use yew::prelude::*;
use yew_router::prelude::*;

use crate::AnimechanQuote;
use crate::route::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct QuoteProp {
    pub quote: Box<AnimechanQuote>,
    #[prop_or(true)]
    pub header: bool,
    #[prop_or(true)]
    pub footer: bool,
}

#[function_component(QuoteComponent)]
pub fn quote(props: &QuoteProp) -> Html {
    let animechan_quote = props.quote.as_ref();
    let quote = animechan_quote.get_quote();
    let anime = animechan_quote.get_anime();
    let character = animechan_quote.get_character();

    let header = if props.header {
        let anime_route = Route::Anime {
            title: props.header.to_string(),
        };
        html! {
            <div class="card-header">
                <Link<Route> to={anime_route}>{ anime }</Link<Route>>
            </div>
        }
    } else {
        Html::default()
    };

    let footer = if props.footer {
        let character_route = Route::Character {
            character: props.quote.get_character().to_string(),
        };
        html! {
            <footer class="blockquote-footer">
                <Link<Route> to={character_route}>{ character }</Link<Route>>
            </footer>
        }
    } else {
        Html::default()
    };

    html! {
        <div class="card rounded-3 shadow-sm m-1">
            {header}
            <div class="card-body">
                <blockquote class = "blockquote mb-0">
                    <p class="card-text">{quote}</p>
                    {footer}
                </blockquote>
            </div>
        </div>
    }
}
