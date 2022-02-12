use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use crate::error::*;
use crate::loading::*;

#[async_trait(? Send)]
pub trait ViewAsync<ELEMENT>
where
    ELEMENT: Debug + PartialEq,
    Self: PartialEq + Clone,
{
    async fn fetch_data(&self) -> Message<ELEMENT>;
    fn successful_view(
        &self,
        _element: Rc<ELEMENT>,
    ) -> Html {
        Html::default()
    }
    fn failed_view(
        &self,
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
    fn loading_view(&self) -> Html {
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
    Failed(Rc<dyn Error>),
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
pub struct AsyncProp<ELEMENT, PROVIDER>
    where
        ELEMENT: Debug + PartialEq,
        PROVIDER: PartialEq + Clone + ViewAsync<ELEMENT>,
{
    pub provider: PROVIDER,
    #[prop_or_default]
    phantom: PhantomData<ELEMENT>,
}

#[function_component(AsyncComponent)]
pub fn async_component<ELEMENT, PROVIDER>(props: &AsyncProp<ELEMENT, PROVIDER>) -> Html
    where
        ELEMENT: Debug + PartialEq + 'static,
        PROVIDER: PartialEq + Clone + ViewAsync<ELEMENT> + 'static,
{
    let state: UseStateHandle<Message<ELEMENT>> = use_state(|| Message::Loading);
    let provider = &props.provider;
    {
        let state = state.clone();
        let provider = provider.clone();
        use_effect_with_deps(move |_| {
            let state = state.clone();
            let provider = provider.clone();
            wasm_bindgen_futures::spawn_local(async move { state.set(provider.fetch_data().await); });
            || ()
        }, ());
    };
    match state.deref() {
        Message::Loading => provider.loading_view(),
        Message::Successful(x) => provider.successful_view(x.clone()),
        Message::Failed(x) => provider.failed_view(x.clone()),
    }
}
