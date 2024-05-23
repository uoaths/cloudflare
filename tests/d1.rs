mod common;

#[cfg(feature = "d1")]
mod tests_d1 {
    const MOCK_DATABASE_ID: &str = "1a9ab71c-14b6-4dba-8397-4484a4143953";

    use super::common;

    #[tokio::test]
    async fn test_fetch_query_d1_database_empty_enter() {
        common::server().mock(|when, then| {
            when.method("POST")
                .path(format!(
                    "/client/v4/accounts/{}/d1/database/{}/query",
                    common::MOCK_ACCOUNT_ID,
                    MOCK_DATABASE_ID
                ))
                .body(
                    r#"{
                "sql": "",
                "params": "[]"
            }"#,
                );
            then.status(400).body(
                r#"{
                "result": null,
                "success": false,
                "errors": [
                  {
                    "code": 7400,
                    "message": "The request is malformed: no query"
                  }
                ],
                "messages": []
              }"#,
            );
        });

        common::client()
            .query_d1_database(MOCK_DATABASE_ID, "", &[""])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_fetch_query_d1_database_create_table() {
        common::server().mock(|when, then| {
            when.method("POST").path(format!(
                "/client/v4/accounts/{}/d1/database/{}/query",
                common::MOCK_ACCOUNT_ID,
                MOCK_DATABASE_ID
            ));
            then.status(200).body(
                r#"{
                "result": [
                    {
                        "results": [],
                        "success": true,
                        "meta": {
                            "served_by": "v3-prod",
                            "duration": 0.31,
                            "changes": 0,
                            "last_row_id": 0,
                            "changed_db": true,
                            "size_after": 36864,
                            "rows_read": 1,
                            "rows_written": 4
                        }
                    }
                ],
                "success": true,
                "errors": [],
                "messages": []
            }"#,
            );
        });

        common::client()
            .query_d1_database(
                MOCK_DATABASE_ID,
                "CREATE TABLE IF NOT EXISTS Person (
                serial      INTEGER PRIMARY KEY NOT NULL,
                identifier  TEXT                NOT NULL  UNIQUE,
                name        TEXT                NOT NULL  UNIQUE,
                role        INTEGER             NOT NULL,
                password    TEXT                NOT NULL )",
                &[],
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_fetch_query_d1_database_insert_sql() {
        common::server().mock(|when, then| {
            when.method("POST").path(format!(
                "/client/v4/accounts/{}/d1/database/{}/query",
                common::MOCK_ACCOUNT_ID,
                MOCK_DATABASE_ID
            ));
            then.status(200).body(
                r#"{
                    "result": [
                        {
                            "results": [],
                            "success": true,
                            "meta": {
                                "served_by": "v3-prod",
                                "duration": 0.2036,
                                "changes": 1,
                                "last_row_id": 4,
                                "changed_db": true,
                                "size_after": 36864,
                                "rows_read": 0,
                                "rows_written": 3
                            }
                        }
                    ],
                    "success": true,
                    "errors": [],
                    "messages": []
                }"#,
            );
        });

        common::client()
            .query_d1_database(
                MOCK_DATABASE_ID,
                "INSERT INTO Person (identifier, name, role, password) VALUES ($1, $2, $3, $4)",
                &["id1", "name1", "1", "password1"],
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_fetch_query_d1_database_select_sql() {
        common::server().mock(|when, then| {
            when.method("POST").path(format!(
                "/client/v4/accounts/{}/d1/database/{}/query",
                common::MOCK_ACCOUNT_ID,
                MOCK_DATABASE_ID
            ));
            then.status(200).body(
                r#"{
                    "result": [
                        {
                            "results": [
                                {
                                    "serial": 1,
                                    "identifier": "id1",
                                    "name": "name1",
                                    "role": 1,
                                    "password": "password1"
                                },
                                {
                                    "serial": 2,
                                    "identifier": "id2",
                                    "name": "name2",
                                    "role": 2,
                                    "password": "password2"
                                }
                            ],
                            "success": true,
                            "meta": {
                                "served_by": "v3-prod",
                                "duration": 0.2528,
                                "changes": 0,
                                "last_row_id": 0,
                                "changed_db": false,
                                "size_after": 36864,
                                "rows_read": 4,
                                "rows_written": 0
                            }
                        }
                    ],
                    "success": true,
                    "errors": [],
                    "messages": []
                }"#,
            );
        });

        common::client()
            .query_d1_database(MOCK_DATABASE_ID, "SELECT * FROM PERSON", &[])
            .await
            .unwrap();
    }
}
