use reqwest::Client;

#[derive(Clone, Debug)]
pub struct Context {
    client: Client,
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            client: Client::new(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
