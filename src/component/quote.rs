use yew::prelude::*;

use crate::AnimechanQuote;

#[derive(Properties, PartialEq, Clone)]
pub struct QuoteProp {
    pub quote: Box<AnimechanQuote>,
    pub header: bool,
}

pub struct QuoteComponent;

impl Component for QuoteComponent {
    type Message = ();
    type Properties = QuoteProp;

    fn create(ctx: &Context<Self>) -> Self {
        QuoteComponent
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let prop = ctx.props().quote.as_ref();
        let quote = prop.get_quote();
        let anime = prop.get_anime();
        let character = prop.get_character();

        let header = if ctx.props().header {
            html! {<div class="card-header">{anime}</div>}
        } else {
            Html::default()
        };

        html! {
            <div class="card rounded-3 shadow-sm m-1">
                {header}
                <div class="card-body">
                    <blockquote class = "blockquote mb-0">
                        <p class="card-text">
                            {quote}
                        </p>
                        <footer class="blockquote-footer">{character}</footer>
                    </blockquote>
                </div>
            </div>
        }
    }
}
