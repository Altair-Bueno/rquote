use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AnimeProp {
    pub title: String,
}

#[function_component(Anime)]
pub fn anime(anime_prop: &AnimeProp) -> Html {
    let title = anime_prop.title.as_str();
    html! {
        <p>{title}</p>
    }
}
