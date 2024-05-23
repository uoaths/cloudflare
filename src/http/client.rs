use std::error::Error;
use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::Client as RequestClient;

use super::error::ClientError;
use super::request::Secret;

const DEFAULT_BASE_URL: &str = "https://api.cloudflare.com/";

pub type ClientResult<T> = Result<T, ClientError>;

pub struct Client {
    pub(crate) base_url: String,
    pub(crate) inner: RequestClient,
    pub(crate) secret: Secret,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

pub struct ClientBuilder {
    base_url: String,
    secret: Secret,
    timeout: Duration,
    header: HeaderMap,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        let mut default_header = HeaderMap::new();
        default_header.insert("Content-Type", "application/json".parse().unwrap());

        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            secret: Secret::default(),
            timeout: Duration::from_secs(30),
            header: default_header,
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Client, Box<dyn Error>> {
        let client = RequestClient::builder()
            .connect_timeout(self.timeout)
            .default_headers(self.header);

        let client = Client {
            inner: client.build()?,
            secret: self.secret,
            base_url: self.base_url,
        };

        Ok(client)
    }

    pub fn set_base_url(mut self, value: String) -> Self {
        self.base_url = value;

        self
    }

    pub fn set_timeout(mut self, value: Duration) -> Self {
        self.timeout = value;

        self
    }

    pub fn set_api_key(mut self, value: String) -> Self {
        self.secret.update_api_key(value);

        self
    }

    pub fn set_api_email(mut self, value: String) -> Self {
        self.secret.update_api_email(value);

        self
    }

    pub fn set_bearer_auth(mut self, value: String) -> Self {
        self.secret.update_bearer_auth(value);

        self
    }

    pub fn set_account_id(mut self, value: String) -> Self {
        self.secret.update_account_id(value);

        self
    }
}
