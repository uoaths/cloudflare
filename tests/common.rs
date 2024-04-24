use httpmock::MockServer;
use once_cell::sync::Lazy;

use cloudflare::prelude::*;

// Fake
pub const MOCK_API_KEY: &str = "1e9527c0e41f9f0bd49527e3fd9b41";
pub const MOCK_API_EMAIL: &str = "test@cloudflare.com";
pub const MOCK_BEARER_AUTH: &str = "FakebqAH_Am1_KvqjseOlki475rAZczLWntgT8";
pub const MOCK_ACCOUNT_ID: &str = "594156d959527ad0d071722946d17c7";

pub fn client() -> Client {
    Client::builder()
        .set_base_url(MOCK_SERVER.base_url())
        .set_api_key(MOCK_API_KEY.into())
        .set_bearer_auth(MOCK_BEARER_AUTH.into())
        .set_api_email(MOCK_API_EMAIL.into())
        .set_account_id(MOCK_ACCOUNT_ID.into())
        .build()
        .unwrap()
}

pub fn server() -> &'static MockServer {
    &*MOCK_SERVER
}

pub static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| {
    println!("Starting mock server...");
    MockServer::start()
});
