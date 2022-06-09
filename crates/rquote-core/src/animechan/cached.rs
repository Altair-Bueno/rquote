use cached::proc_macro::cached;
use cached::SizedCache;
use reqwest::Client;
use reqwest::Result;

use super::ANIMECHAN_ANIME_LIST;
use super::ANIMECHAN_CHARACTER_QUOTE;
use super::ANIMECHAN_TITLE_QUOTE;
use super::AnimechanQuote;

const CACHE_MAX_ITEMS : usize = 20;

#[cached(
    result=true,
    type = "SizedCache<String, Vec<AnimechanQuote>>",
    create = "{ SizedCache::with_size(CACHE_MAX_ITEMS) }",
    convert = r#"{ format!("{}-{}", title, page) }"#
)]
pub async fn get_quote_title(
    client: &Client,
    title: &str,
    page: u32,
) -> Result<Vec<AnimechanQuote>> {
    let page = page.to_string();
    let parameter = [("title", title), ("page", &page)];
    client
        .get(ANIMECHAN_TITLE_QUOTE)
        .query(&parameter)
        .send()
        .await?
        .json()
        .await
}

#[cached(
    result=true,
    type = "SizedCache<String, Vec<AnimechanQuote>>",
    create = "{ SizedCache::with_size(CACHE_MAX_ITEMS) }",
    convert = r#"{ format!("{}-{}", character, page) }"#
)]
pub async fn get_quote_character(
    client: &Client,
    character: &str,
    page: u32,
) -> Result<Vec<AnimechanQuote>> {
    let page = page.to_string();
    let parameter = [("name", character), ("page", &page)];
    client
        .get(ANIMECHAN_CHARACTER_QUOTE)
        .query(&parameter)
        .send()
        .await?
        .json()
        .await
}

// () has 0 size
#[cached(
    result=true,
    type = "SizedCache<(), Vec<String>>",
    create = "{ SizedCache::with_size(1) }",
    convert = r#"{ () }"#
)]
pub async fn get_anime_list(client: &Client) -> Result<Vec<String>> {
    client.get(ANIMECHAN_ANIME_LIST).send().await?.json().await
}
