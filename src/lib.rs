mod request;
mod response;

pub mod client;

#[cfg(feature="d1")]
pub mod d1;

pub mod error;

pub mod prelude {
    pub use super::client::{Client, ClientBuilder, ClientResult};
    pub use super::error::ClientError;
}

pub mod types {
    #[cfg(feature="d1")]
    pub use super::d1::types::*;

    pub use super::response::{CodeMessage, Response};
}
