pub mod auth {
    use crate::database::db_work::get_connection;
    use async_std::{print, println};
    use serde::{Deserialize, Serialize};
    use tide::{http::Response, ResponseBuilder};
    use tokio_postgres::{row, Client, GenericClient, NoTls};

    #[derive(Deserialize, Serialize)]
    struct Creds {
        username: String,
        password: String,
    }

    pub async fn login(
        mut req: tide::Request<()>,
        conn_str: String,
    ) -> tide::Result<tide::Response> {
        let creds: Creds = req
            .body_json()
            .await
            .expect("error reading or parsing body");
        let client = get_connection(conn_str).await;

        let rows = client
            .query(
                "SELECT * FROM users WHERE password = $1 and username = $2;",
                &[
                    &creds.password.replace("\"", "'"),
                    &creds.username.replace("\"", "'"),
                ],
            )
            .await;

        match rows {
            Ok(rows) => {
                let mut response = tide::Response::new(tide::StatusCode::Ok);

                if rows.len() == 1 {
                    response.set_body("legit");

                    Ok(response)
                } else {
                    let resp = tide::Response::new(tide::StatusCode::Unauthorized);
                    Ok(resp.into())
                }
            }
            Err(e) => {
                let mut resp = tide::Response::new(tide::StatusCode::Unauthorized);
                let serialized = serde_json::to_string(&creds).unwrap();
                resp.set_body(e.to_string());
                Ok(resp.into())
            }
        }
    }
}
