use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

use async_trait::async_trait;
use reqwest::{Client, Error as ReqwestError};
use yew::prelude::*;

use rquote_core::wrapper::ClientWrapper;

use crate::error::*;
use crate::loading::*;

#[async_trait(? Send)]
pub trait ViewAsync<ELEMENT>
where
    ELEMENT: Debug + PartialEq,
    Self: PartialEq + Clone,
{
    async fn fetch_data(&self, client: Client) -> Message<ELEMENT>;
    fn successful_view(
        &self,
        _ctx: &Context<AsyncComponent<ELEMENT, Self>>,
        _element: Rc<ELEMENT>,
    ) -> Html {
        Html::default()
    }
    fn failed_view(
        &self,
        _ctx: &Context<AsyncComponent<ELEMENT, Self>>,
        error: Rc<dyn Error>,
    ) -> Html {
        let onclick = |_| todo!();
        let _ = html! {
            <button {onclick} class={classes!("btn","btn-light","text-dark")}>
                {"Reload"}
            </button>
        };
        html! {
            <ErrorComponent severity={Severity::Danger} {error}>
                //{reload_button}
            </ErrorComponent>
        }
    }
    fn loading_view(&self, _ctx: &Context<AsyncComponent<ELEMENT, Self>>) -> Html {
        html! {<LoadingComponent/>}
    }
}

#[derive(Debug)]
pub enum Message<ELEMENT>
where
    ELEMENT: Debug + PartialEq,
{
    Loading,
    Successful(Rc<ELEMENT>),
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
pub struct AsyncFetchProp<ELEMENT, PROVIDER>
where
    ELEMENT: Debug + PartialEq,
    PROVIDER: PartialEq + Clone + ViewAsync<ELEMENT>,
{
    pub provider: PROVIDER,
    #[prop_or_default]
    phantom: PhantomData<ELEMENT>,
}

#[derive(Debug)]
pub struct AsyncComponent<ELEMENT, PROVIDER>
where
    ELEMENT: Debug + PartialEq + 'static,
    PROVIDER: PartialEq + Clone + ViewAsync<ELEMENT> + 'static,
{
    message: Message<ELEMENT>,
    phantom: PhantomData<PROVIDER>,
}

impl<ELEMENT, PROVIDER> Component for AsyncComponent<ELEMENT, PROVIDER>
where
    ELEMENT: Debug + PartialEq + 'static,
    PROVIDER: PartialEq + Clone + ViewAsync<ELEMENT> + 'static,
{
    type Message = Message<ELEMENT>;
    type Properties = AsyncFetchProp<ELEMENT, PROVIDER>;

    fn create(ctx: &Context<Self>) -> Self {
        let client = ctx
            .link()
            .context::<ClientWrapper>(Default::default())
            .map(|x| x.0)
            .unwrap_or_default()
            .take();
        let provider = ctx.props().provider.clone();
        ctx.link()
            .callback_future_once(|_| async move { provider.fetch_data(client).await })
            .emit(());
        AsyncComponent {
            message: Message::default(),
            phantom: Default::default(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            x => {
                self.message = x;
                true
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let provider = &ctx.props().provider;
        match &self.message {
            Message::Loading => provider.loading_view(ctx),
            Message::Successful(x) => provider.successful_view(ctx, x.clone()),
            Message::Failed(x) => provider.failed_view(ctx, x.clone()),
        }
    }
}
