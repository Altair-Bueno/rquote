use reqwest::Client;
use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::animechan::cached::{
    get_anime_list,
    get_quote_character,
    get_quote_title
};

mod cached;

const ANIMECHAN_RANDOM_QUOTE: &str = "https://animechan.vercel.app/api/random";
const ANIMECHAN_10_RANDOM_QUOTE: &str = "https://animechan.vercel.app/api/quotes";
const ANIMECHAN_TITLE_QUOTE: &str = "https://animechan.vercel.app/api/quotes/anime";
const ANIMECHAN_CHARACTER_QUOTE: &str = "https://animechan.vercel.app/api/quotes/character";
const ANIMECHAN_ANIME_LIST: &str = "https://animechan.vercel.app/api/available/anime";

/// Type that can be Serialized and Deserialized as a valid Animechan quote. You
/// can find more information about Animechan's API
/// [here](https://animechan.vercel.app/)
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AnimechanQuote {
    anime: String,
    character: String,
    quote: String,
}

impl AnimechanQuote {
    pub async fn get_random_quote(client: &Client) -> Result<AnimechanQuote> {
        client
            .get(ANIMECHAN_RANDOM_QUOTE)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_10_random_quotes(client: &Client) -> Result<Vec<AnimechanQuote>> {
        client
            .get(ANIMECHAN_10_RANDOM_QUOTE)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_quote_title(
        client: &Client,
        title: &str,
        page: Option<u32>,
    ) -> Result<Vec<AnimechanQuote>> { get_quote_title(client,title,page).await }

    pub async fn get_quote_character(
        client: &Client,
        character: &str,
        page: Option<u32>,
    ) -> Result<Vec<AnimechanQuote>> { get_quote_character(client,character,page).await }

    pub async fn get_anime_list(client: &Client) -> Result<Vec<String>> { get_anime_list(client).await }

    pub fn anime(&self) -> &str {
        self.anime.as_str()
    }
    pub fn character(&self) -> &str {
        self.character.as_str()
    }
    pub fn quote(&self) -> &str {
        self.quote.as_str()
    }
}

#[cfg(test)]
mod test {
    use reqwest::Client;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::animechan::AnimechanQuote;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub async fn random_quote() {
        let client = Client::new();
        let response = AnimechanQuote::get_random_quote(&client).await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn random_10_quotes() {
        let client = Client::new();
        let response = AnimechanQuote::get_10_random_quotes(&client).await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn anime_list() {
        let client = Client::new();
        let response = AnimechanQuote::get_anime_list(&client).await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn quote_title() {
        let client = Client::new();
        let response = AnimechanQuote::get_quote_title(&client, "Hyouka", None).await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn quote_character() {
        let client = Client::new();
        let response = AnimechanQuote::get_quote_character(&client, "Saitama", None).await;
        assert!(matches!(response, Ok(_)))
    }
}
