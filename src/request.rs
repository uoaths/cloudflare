use serde::{Deserialize, Serialize};
use url::Url;

use super::client::{Client, ClientResult};
use super::error::ClientError;

impl Client {
    pub fn base_url(&self) -> ClientResult<Url> {
        Ok(Url::parse(&self.base_url)?)
    }

    pub(crate) fn build_request_get(&self, url: Url) -> RequestBuilder {
        let inner = self.inner.get(url);

        RequestBuilder { inner }
    }

    pub(crate) fn build_request_post(&self, url: Url) -> RequestBuilder {
        let inner = self.inner.post(url);

        RequestBuilder { inner }
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
pub(crate) struct Secret {
    api_key:     Option<String>,
    api_email:   Option<String>,
    bearer_auth: Option<String>,
    account_id:  Option<String>,
}

impl Secret {
    pub(crate) fn api_key(&self) -> ClientResult<&String> {
        match &self.api_key {
            Some(value) => Ok(value),
            None => Err(ClientError::Authorization("API key".into())),
        }
    }

    pub(crate) fn api_email(&self) -> ClientResult<&String> {
        match &self.api_email {
            Some(value) => Ok(value),
            None => Err(ClientError::Authorization("API email".into())),
        }
    }

    pub(crate) fn bearer_auth(&self) -> ClientResult<&String> {
        match &self.bearer_auth {
            Some(value) => Ok(value),
            None => Err(ClientError::Authorization("Bearer auth".into())),
        }
    }

    pub(crate) fn account_id(&self) -> ClientResult<&String> {
        match &self.account_id {
            Some(value) => Ok(value),
            None => Err(ClientError::Authorization("Account id".into())),
        }
    }

    pub fn update_api_key(&mut self, value: String) {
        self.api_key = Some(value)
    }

    pub fn update_api_email(&mut self, value: String) {
        self.api_email = Some(value)
    }

    pub fn update_bearer_auth(&mut self, value: String) {
        self.bearer_auth = Some(value)
    }

    pub fn update_account_id(&mut self, value: String) {
        self.account_id = Some(value)
    }
}

#[derive(Debug)]
pub(crate) struct RequestBuilder {
    inner: reqwest::RequestBuilder,
}

impl From<reqwest::RequestBuilder> for RequestBuilder {
    fn from(value: reqwest::RequestBuilder) -> Self {
        Self { inner: value }
    }
}

impl RequestBuilder {
    pub(crate) async fn send<T>(self) -> ClientResult<T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let response = self.inner.send().await?;

        Ok(response.json::<T>().await?)
    }

    pub(crate) fn with_bearer_auth(self, value: &String) -> Self {
        self.inner.bearer_auth(value).into()
    }

    pub(crate) fn with_api_key(self, value: &String) -> Self {
        self.inner.header("X-Auth-Key", value).into()
    }

    pub(crate) fn with_api_email(self, value: &String) -> Self {
        self.inner.header("X-Auth-Email", value).into()
    }

    pub(crate) fn with_json_body<T>(self, value: &T) -> Self
    where
        T: Serialize,
    {
        self.inner.json(value).into()
    }
}
