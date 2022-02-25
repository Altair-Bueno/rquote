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
    #[prop_or_default]
    pub class: Classes,
}

/// A quote component shows a quote with a Bootstrap card
///
/// ```rust
/// use yew::prelude::*;
/// use rquote::custom::*;
/// use rquote_core::AnimechanQuote;
/// use reqwest::Client;
///
/// #[function_component(App)]
/// fn app()->Html {
///     let quote = AnimechanQuote::get_random_quote(&Client::default());
///     let header = true;
///     let footer = true;
///     let class = classes!("bg-primary");
///     let provider = Example;
///     html!{
///         <QuoteComponent {quote} {header} {footer} {class} />
///     }
/// }
/// ```
#[function_component(QuoteComponent)]
pub fn quote(props: &QuoteProp) -> Html {
    let animechan_quote = &props.quote;
    let quote = animechan_quote.quote();
    let anime = animechan_quote.anime();
    let character = animechan_quote.character();
    let theme = use_context::<Theme>().unwrap_or_default();

    let header = if props.header {
        let anime_route = Route::Anime {
            title: urlencoding::encode(props.quote.anime()).to_string(),
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
            character: urlencoding::encode(props.quote.character()).to_string()
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
    let mut class = props.class.clone();
    class.push("card");
    class.push("rounded-3");
    class.push("shadow-sm");
    class.push(theme.get_background_class());
    html! {
        <div {class}>
            {header}
            <div class={classes!("card-body")}>
                <blockquote class={classes!("blockquote")}>
                    <p class={classes!("card-text",theme.get_text_class())}>{quote}</p>
                    {footer}
                </blockquote>
            </div>
        </div>
    }
}
