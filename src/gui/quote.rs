use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget};

use crate::animechan::AnimechanQuote;

pub struct QuoteWidget {
    quote: AnimechanQuote,
}

impl QuoteWidget {
    pub fn new(quote: AnimechanQuote) -> QuoteWidget {
        QuoteWidget { quote }
    }
}
