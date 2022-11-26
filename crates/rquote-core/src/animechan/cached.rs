use cached::proc_macro::cached;
use cached::SizedCache;
use gloo_net::http::Request;
use gloo_net::Error;

use super::AnimechanQuote;
use super::ANIMECHAN_ANIME_LIST;
use super::ANIMECHAN_CHARACTER_QUOTE;
use super::ANIMECHAN_TITLE_QUOTE;

const CACHE_MAX_ITEMS: usize = 20;

#[cached(
    result = true,
    type = "SizedCache<String, Vec<AnimechanQuote>>",
    create = "{ SizedCache::with_size(CACHE_MAX_ITEMS) }",
    convert = r#"{ format!("{}-{}", title, page) }"#
)]
pub async fn get_quote_title(title: &str, page: u32) -> Result<Vec<AnimechanQuote>, Error> {
    let page = page.to_string();
    Request::get(ANIMECHAN_TITLE_QUOTE)
        .query([("title", title), ("page", &page)])
        .send()
        .await?
        .json()
        .await
}

#[cached(
    result = true,
    type = "SizedCache<String, Vec<AnimechanQuote>>",
    create = "{ SizedCache::with_size(CACHE_MAX_ITEMS) }",
    convert = r#"{ format!("{}-{}", character, page) }"#
)]
pub async fn get_quote_character(character: &str, page: u32) -> Result<Vec<AnimechanQuote>, Error> {
    let page = page.to_string();
    Request::get(ANIMECHAN_CHARACTER_QUOTE)
        .query([("name", character), ("page", &page)])
        .send()
        .await?
        .json()
        .await
}

// () has 0 size
#[cached(
    result = true,
    type = "SizedCache<(), Vec<String>>",
    create = "{ SizedCache::with_size(1) }",
    convert = r#"{ () }"#
)]
pub async fn get_anime_list() -> Result<Vec<String>, Error> {
    Request::get(ANIMECHAN_ANIME_LIST)
        .send()
        .await?
        .json()
        .await
}
