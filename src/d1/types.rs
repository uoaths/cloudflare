use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub enum D1DatabaseVersion {
    #[serde(rename(deserialize = "alpha"))]
    Alpha,

    #[serde(rename(deserialize = "beta"))]
    Beta,

    #[serde(rename(deserialize = "production"))]
    Production,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct ListD1DatabasesResult {
    // D1 database name
    pub name: String,

    // D1 database uuid
    pub uuid: String,

    // Specifies the timestamp the resource was created as an ISO8601 string
    pub created_at: String,

    // D1 database version
    pub version: D1DatabaseVersion
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct D1DatabaseMeta {
    pub served_by:    String,
    pub changed_db:   bool,
    pub changes:      f64,
    pub duration:     f64,
    pub rows_read:    f64,
    pub last_row_id:  f64,
    pub rows_written: f64,
    pub size_after:   f64,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct QueryD1DatabaseResult {
    pub meta:    D1DatabaseMeta,
    pub success: bool,
    pub results: Vec<Value>,
}
