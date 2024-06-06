pub mod auth {
    use crate::database::db_work::get_connection;

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Creds {
        username: String,
        password: String,
    }

    #[derive(Deserialize, Serialize)]
    struct CredsReg {
        username: String,
        password: String,
        mail: String,
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
                let _serialized = serde_json::to_string(&creds).unwrap();
                resp.set_body(e.to_string());
                Ok(resp.into())
            }
        }
    }

    pub async fn register(
        mut req: tide::Request<()>,
        conn_str: String,
    ) -> tide::Result<tide::Response> {
        let client = get_connection(conn_str).await;

        let creds: CredsReg = req
            .body_json()
            .await
            .expect("error reading or parsing body");

        let rows = client
            .query(
                "INSERT INTO users (username, password, email) VALUES ($1, $2, $3);",
                &[
                    &creds.username.replace("\"", "'"),
                    &creds.password.replace("\"", "'"),
                    &creds.mail.replace("\"", "'"),
                ],
            )
            .await;

        match rows {
            Ok(_) => Ok(tide::Response::new(tide::StatusCode::Ok)),
            Err(_) => Ok(tide::Response::new(tide::StatusCode::Unauthorized)),
        }
    }
}
