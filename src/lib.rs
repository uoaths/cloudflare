mod http;

#[cfg(feature = "d1")]
pub mod d1;

pub mod prelude {
    pub use super::http::client::{Client, ClientBuilder, ClientResult};
    pub use super::http::error::ClientError;
}
