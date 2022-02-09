use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use yew::prelude::*;

#[async_trait(? Send)]
pub trait ViewAsyncListComponent<ELEMENT>
where
    ELEMENT: Debug + PartialEq,
    Self: PartialEq + Clone,
{
    async fn fetch_data(&self, client: Client) -> Message<ELEMENT>;
    fn successful_view(
        &self,
        ctx: &Context<AsyncListComponent<ELEMENT, Self>>,
        quotes: &[ELEMENT],
    ) -> Html;
    fn failed_view(&self, ctx: &Context<AsyncListComponent<ELEMENT, Self>>, error: Rc<dyn Error>) -> Html;
    fn loading_view(&self, ctx: &Context<AsyncListComponent<ELEMENT, Self>>) -> Html;
}

#[derive(Debug)]
pub enum Message<ELEMENT>
    where
        ELEMENT: Debug + PartialEq,
{
    Loading,
    Successful(Vec<ELEMENT>),
    Failed(Rc<ReqwestError>),
}

impl<ELEMENT> Default for Message<ELEMENT>
    where
        ELEMENT: Debug + PartialEq,
{
    fn default() -> Self {
        Message::Loading
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AsyncListProp<ELEMENT, PROVIDER>
    where
        ELEMENT: Debug + PartialEq,
        PROVIDER: PartialEq + Clone + ViewAsyncListComponent<ELEMENT>,
{
    pub provider: PROVIDER,
    #[prop_or_default]
    phantom: PhantomData<ELEMENT>,
}

#[derive(Debug)]
pub struct AsyncListComponent<ELEMENT, PROVIDER>
    where
        ELEMENT: Debug + PartialEq + 'static,
        PROVIDER: PartialEq + Clone + ViewAsyncListComponent<ELEMENT> + 'static,
{
    quotes: Message<ELEMENT>,
    phantom: PhantomData<PROVIDER>,
}

impl<ELEMENT, PROVIDER> Component for AsyncListComponent<ELEMENT, PROVIDER>
    where
        ELEMENT: Debug + PartialEq + 'static,
        PROVIDER: PartialEq + Clone + ViewAsyncListComponent<ELEMENT> + 'static,
{
    type Message = Message<ELEMENT>;
    type Properties = AsyncListProp<ELEMENT, PROVIDER>;

    fn create(ctx: &Context<Self>) -> Self {
        let client = ctx
            .link()
            .context::<crate::wrapper::ClientContext>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default()
            .take();
        let provider = ctx.props().provider.clone();
        ctx.link()
            .callback_future_once(|_| async move {
                provider.fetch_data(client).await
            })
            .emit(());
        AsyncListComponent {
            quotes: Message::default(),
            phantom: Default::default(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            x => {
                self.quotes = x;
                true
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let provider = &ctx.props().provider;
        match &self.quotes {
            Message::Loading => provider.loading_view(ctx),
            Message::Successful(x) => provider.successful_view(ctx, x.as_slice()),
            Message::Failed(x) => provider.failed_view(ctx, x.clone()),
        }
    }
}
