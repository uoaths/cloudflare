pub mod types;

use serde_json::json;

use crate::{
    client::{Client, ClientResult},
    types::Response,
};

use self::types::*;

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
