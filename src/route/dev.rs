use std::error::Error;
use std::rc::Rc;

use yew::prelude::*;

use crate::component::error::*;

#[function_component(Dev)]
pub fn dev() -> Html {
    let error = "ddd".parse::<std::net::IpAddr>().unwrap_err();
    let rc: Rc<dyn Error> = Rc::new(error);
    html! {
        <>
        <ErrorComponent severity={Severity::Danger} error = {rc.clone()} />
        <ErrorComponent severity={Severity::Minor} error = {rc.clone()} />
        <ErrorComponent severity={Severity::Warning} error = {rc.clone()} />
        </>
    }
}