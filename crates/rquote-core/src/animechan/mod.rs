use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Deserialize, Serialize};

use crate::animechan::cached::{get_anime_list, get_quote_character, get_quote_title};

mod cached;

const ANIMECHAN_RANDOM_QUOTE: &str = "https://animechan.xyz/api/random";
const ANIMECHAN_10_RANDOM_QUOTE: &str = "https://animechan.xyz/api/quotes";
const ANIMECHAN_TITLE_QUOTE: &str = "https://animechan.xyz/api/quotes/anime";
const ANIMECHAN_CHARACTER_QUOTE: &str = "https://animechan.xyz/api/quotes/character";
const ANIMECHAN_ANIME_LIST: &str = "https://animechan.xyz/api/available/anime";

/// Type that can be Serialized and Deserialized as a valid Animechan quote. You
/// can find more information about Animechan's API
/// [here](https://animechan.xyz/)
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AnimechanQuote {
    anime: String,
    character: String,
    quote: String,
}

impl AnimechanQuote {
    pub async fn get_random_quote() -> Result<AnimechanQuote, Error> {
        Request::get(ANIMECHAN_RANDOM_QUOTE)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_10_random_quotes() -> Result<Vec<AnimechanQuote>, Error> {
        Request::get(ANIMECHAN_10_RANDOM_QUOTE)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_quote_title(title: &str, page: u32) -> Result<Vec<AnimechanQuote>, Error> {
        get_quote_title(title, page).await
    }

    pub async fn get_quote_character(
        character: &str,
        page: u32,
    ) -> Result<Vec<AnimechanQuote>, Error> {
        get_quote_character(character, page).await
    }

    pub async fn get_anime_list() -> Result<Vec<String>, Error> {
        get_anime_list().await
    }

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
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::animechan::AnimechanQuote;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    pub async fn random_quote() {
        let response = AnimechanQuote::get_random_quote().await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn random_10_quotes() {
        let response = AnimechanQuote::get_10_random_quotes().await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn anime_list() {
        let response = AnimechanQuote::get_anime_list().await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn quote_title() {
        let response = AnimechanQuote::get_quote_title("Hyouka", 0).await;
        assert!(matches!(response, Ok(_)))
    }

    #[wasm_bindgen_test]
    pub async fn quote_character() {
        let response = AnimechanQuote::get_quote_character("Saitama", 0).await;
        assert!(matches!(response, Ok(_)))
    }
}
