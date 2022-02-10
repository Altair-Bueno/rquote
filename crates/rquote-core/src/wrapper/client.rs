use reqwest::Client;

#[derive(Clone, Debug, Default)]
pub struct ClientWrapper(Client);

impl PartialEq for ClientWrapper {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl ClientWrapper {
    pub fn new(client: Client) -> Self {
        Self(client)
    }
    // FIXME
    #[allow(dead_code)]
    pub fn as_ref(&self) -> &Client {
        &self.0
    }
    pub fn take(self) -> Client {
        self.0
    }
}
