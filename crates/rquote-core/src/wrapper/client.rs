use std::ops::Deref;

use reqwest::Client;

/// Reqwest Wrapper that includes a `PartialEq` implementation
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
    pub fn as_ref(&self) -> &Client {
        &self.0
    }
    pub fn take(self) -> Client {
        self.0
    }
}

impl Deref for ClientWrapper {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
