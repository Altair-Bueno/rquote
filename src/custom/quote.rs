use yew::prelude::*;
use yew_router::prelude::*;

use rquote_component::Theme;
use rquote_core::AnimechanQuote;

use crate::route::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct QuoteProp {
    pub quote: AnimechanQuote,
    #[prop_or(true)]
    pub header: bool,
    #[prop_or(true)]
    pub footer: bool,
}

#[function_component(QuoteComponent)]
pub fn quote(props: &QuoteProp) -> Html {
    let animechan_quote = &props.quote;
    let quote = animechan_quote.get_quote();
    let anime = animechan_quote.get_anime();
    let character = animechan_quote.get_character();
    let theme = use_context::<Theme>().unwrap_or_default();

    let header = if props.header {
        let anime_route = Route::Anime {
            title: props.quote.get_anime().to_string(),
        };
        html! {
            <div class={classes!("card-header")}>
                <Link<Route> to={anime_route} classes= {classes!(theme.get_link_class())}>
                    { anime }
                </Link<Route>>
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
            <footer class={classes!("blockquote-footer")}>
                <Link<Route> to={character_route} classes = {classes!(theme.get_link_class())}>
                    { character }
                </Link<Route>>
            </footer>
        }
    } else {
        Html::default()
    };
    html! {
        <div class={classes!("card", "rounded-3", "shadow-sm", "m-1",theme.get_background_class())}>
            {header}
            <div class={classes!("card-body")}>
                <blockquote class={classes!("blockquote", "mb-0")}>
                    <p class={classes!("card-text",theme.get_text_class())}>{quote}</p>
                    {footer}
                </blockquote>
            </div>
        </div>
    }
}
