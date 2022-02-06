use reqwest::Client;
use reqwest::Result;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use yew::prelude::Properties;

// TODO remove when code is no longer dead
#[allow(dead_code)]
const ANIMECHAN_RANDOM_QUOTE: &str = "https://animechan.vercel.app/api/random";
const ANIMECHAN_10_RANDOM_QUOTE: &str = "https://animechan.vercel.app/api/quotes";
const ANIMECHAN_TITLE_QUOTE: &str = "https://animechan.vercel.app/api/quotes/anime";
const ANIMECHAN_CHARACTER_QUOTE: &str = "https://animechan.vercel.app/api/quotes/character";
const ANIMECHAN_ANIME_LIST: &str = "https://animechan.vercel.app/api/available/anime";

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimechanQuote {
    anime: String,
    character: String,
    quote: String,
}

impl AnimechanQuote {
    pub async fn get_random_quote(client: &Client) -> Result<AnimechanQuote> {
        client
            .get(ANIMECHAN_RANDOM_QUOTE)
            .send().await?
            .json().await
    }
    pub async fn get_10_random_quotes(client: &Client) -> Result<Vec<AnimechanQuote>> {
        client
            .get(ANIMECHAN_10_RANDOM_QUOTE)
            .send().await?
            .json().await
    }
    pub async fn get_quote_title(client: &Client, title: &str, page: Option<u32>) -> Result<Vec<AnimechanQuote>> {
        let page = page
            .map(|x| x.to_string())
            .unwrap_or_else(|| String::from("0"));
        let parameter = [("character", title), ("page", &page)];
        client
            .get(ANIMECHAN_TITLE_QUOTE)
            .form(&parameter)
            .send().await?
            .json().await
    }
    pub async fn get_quote_character(client: &Client, character: &str, page: Option<u32>) -> Result<Vec<AnimechanQuote>> {
        let page = page
            .map(|x| x.to_string())
            .unwrap_or_else(|| String::from("0"));
        let parameter = [("character", character), ("page", &page)];
        client
            .get(ANIMECHAN_TITLE_QUOTE)
            .form(&parameter)
            .send().await?
            .json().await
    }
    pub async fn get_anime_list(client: &Client) -> Result<Vec<String>> {
        client
            .get(ANIMECHAN_ANIME_LIST)
            .send().await?
            .json().await
    }

    pub fn get_anime(&self) -> &str {
        self.anime.as_str()
    }
    pub fn get_character(&self) -> &str {
        self.character.as_str()
    }
    pub fn get_quote(&self) -> &str {
        self.quote.as_str()
    }
    pub fn get_example() -> AnimechanQuote {
        AnimechanQuote {
            quote: "It's not a question of can or can't. There are some things in life you just do.".to_string(),
            character: "Éclair Farron".to_string(),
            anime: "Final Fantasy XIII".to_string(),
        }
    }
}

/*
#[cfg(test)]
mod test {
    use reqwest::Client;

    use crate::animechan::Animechan;

    #[tokio::test]
    pub async fn random_quote() {
        let client = Client::new();
        let response = Animechan::get_random_quote(&client).await;
        matches!(response,Ok(_));
    }

    #[tokio::test]
    pub async fn random_10_quotes() {
        let client = Client::new();
        let response = Animechan::get_10_random_quotes(&client).await;
        matches!(response,Ok(_));
    }

    #[tokio::test]
    pub async fn anime_list() {
        let client = Client::new();
        let response = Animechan::get_anime_list(&client).await;
        matches!(response,Ok(_));
    }

    #[tokio::test]
    pub async fn quote_title() {
        let client = Client::new();
        let response = Animechan::get_quote_title(&client, "Hyouka", None).await;
        matches!(response,Ok(_));
    }

    #[tokio::test]
    pub async fn quote_character() {
        let client = Client::new();
        let response = Animechan::get_quote_character(&client, "Saitama", None).await;
        matches!(response,Ok(_));
    }
}
 */