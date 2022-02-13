use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use async_trait::async_trait;
use yew::prelude::*;

use crate::button::*;
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
        let onclick: Callback<MouseEvent> = {
            |_| if let Some(window) = web_sys::window() {
                let _ = window.location().reload();
            }
        }.into();
        html! {
            <ErrorComponent severity={Severity::Danger} {error}>
            <div class="container">
                <div class="row">
                    <div class="col text-end">
                        <ButtonComponent {onclick}>
                        {"Reload"}
                        </ButtonComponent>
                    </div>
                </div>
            </div>
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
        use_effect_with_deps(move |provider: &PROVIDER| {
            state.set(Message::Loading);
            let provider = provider.clone();
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move { state.set(provider.fetch_data().await); });
            || ()
        }, provider);
    };
    match state.deref() {
        Message::Loading => provider.loading_view(),
        Message::Successful(x) => provider.successful_view(x.clone()),
        Message::Failed(x) => provider.failed_view(x.clone()),
    }
}
