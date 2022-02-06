use yew::prelude::*;

use crate::component::nav_bar::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterProp {
    pub character: String,
}

#[function_component(Character)]
pub fn home(character_prop: &CharacterProp) -> Html {
    html! {
        <p>{character_prop.character.as_str()}</p>
    }
}
