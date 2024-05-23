use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::http::client::{Client, ClientResult};
use crate::http::response::Response;

impl Client {
    // https://developers.cloudflare.com/api/operations/cloudflare-d1-create-database
    pub async fn create_d1_database(
        &self,
        name: &String,
    ) -> ClientResult<Response<ListD1DatabasesResult>> {
        let mut url = self.base_url()?;
        url.set_path(&format!(
            "client/v4/accounts/{}/d1/database",
            self.secret.account_id()?
        ));

        self.build_request_post(url)
            .with_api_key(self.secret.api_key()?)
            .with_api_email(self.secret.api_email()?)
            .with_bearer_auth(self.secret.bearer_auth()?)
            .with_json_body(&json!({
                "name": name
            }))
            .send()
            .await
    }

    // https://developers.cloudflare.com/api/operations/cloudflare-d1-list-databases
    pub async fn list_d1_database(
        &self,
        name: Option<&str>,
        page: Option<usize>,
        per_page: Option<u16>,
    ) -> ClientResult<Response<Vec<ListD1DatabasesResult>>> {
        let mut url = self.base_url()?;
        url.set_path(&format!(
            "client/v4/accounts/{}/d1/database",
            self.secret.account_id()?
        ));

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(name) = name {
                query_pairs.append_pair("name", name);
            }
            if let Some(page) = page {
                query_pairs.append_pair("page", &page.to_string());
            }
            if let Some(per_page) = per_page {
                query_pairs.append_pair("per_page", &per_page.to_string());
            }
        }

        self.build_request_get(url)
            .with_api_key(self.secret.api_key()?)
            .with_api_email(self.secret.api_email()?)
            .with_bearer_auth(self.secret.bearer_auth()?)
            .send()
            .await
    }

    // https://developers.cloudflare.com/api/operations/cloudflare-d1-query-database
    pub async fn query_d1_database(
        &self,
        id: &str,
        sql: &str,
        params: &[&str],
    ) -> ClientResult<Response<Vec<QueryD1DatabaseResult>>> {
        let mut url = self.base_url()?;
        url.set_path(&format!(
            "client/v4/accounts/{}/d1/database/{}/query",
            self.secret.account_id()?,
            id
        ));

        self.build_request_post(url)
            .with_api_key(self.secret.api_key()?)
            .with_api_email(self.secret.api_email()?)
            .with_bearer_auth(self.secret.bearer_auth()?)
            .with_json_body(&json!({
                "params": params,
                "sql": sql
            }))
            .send()
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum D1DatabaseVersion {
    #[serde(rename(deserialize = "alpha"))]
    Alpha,

    #[serde(rename(deserialize = "beta"))]
    Beta,

    #[serde(rename(deserialize = "production"))]
    Production,
}

#[rustfmt::skip]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryD1DatabaseResult {
    pub meta:    D1DatabaseMeta,
    pub success: bool,
    pub results: Vec<Value>,
}

impl QueryD1DatabaseResult {
    pub fn results<T>(self) -> ClientResult<Vec<T>>
    where
        T: for<'a> Deserialize<'a>,
    {
        if self.results.is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::with_capacity(self.results.len());
        for value in self.results {
            result.push(serde_json::from_value::<T>(value).unwrap())
        }

        Ok(result)
    }
}
