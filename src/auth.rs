pub mod auth {

    use core::panic;

    use serde::{Deserialize, Serialize};
    use tokio_postgres::{Client, GenericClient, NoTls};

    #[derive(Deserialize, Serialize, Clone)]
    struct Creds {
        username: String,
        password: String,
    }

    #[derive(Deserialize, Serialize, Clone)]
    pub struct CredsReg {
        username: String,
        password: String,
        mail: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct TokenClaims {
        pub username: String,
        pub mail: String,
        pub exp: usize,
    }

    pub async fn get_connection(conn_str: String) -> Client {
        // Create a connection string

        // Parse the connection string
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();

        // Spawn a task to process the connection
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
    }

    pub async fn get_token(username: String, password: String, conn_str: String) -> String {
        let exp = (chrono::Utc::now().naive_utc() + chrono::naive::Days::new(1))
            .and_utc()
            .timestamp() as usize;

        let data = get_info(Creds { username, password }, conn_str).await;

        let claims = TokenClaims {
            username: data.username,
            mail: data.mail,
            exp,
        };

        let jwt = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("test".as_bytes()),
        )
        .unwrap();

        jwt
    }

    async fn get_info(lc: Creds, conn_str: String) -> CredsReg {
        let client = get_connection(conn_str).await;

        let res = client
            .query(
                "SELECT * FROM users WHERE username = $1 AND password = $2",
                &[&lc.username, &lc.password],
            )
            .await
            .unwrap();

        if res.len() > 1 {
            panic!("how that this happened");
        }

        let info: CredsReg = CredsReg {
            username: res[0].get(0),
            password: res[0].get(1),
            mail: res[0].get(2),
        };

        info
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
        let client = get_connection(conn_str.clone()).await;

        let creds: CredsReg = req
            .body_json()
            .await
            .expect("error reading or parsing body");

        let creds_clone = creds.clone();
        let check = client
            .query(
                "SELECT * FROM users WHERE username = $1",
                &[&creds_clone.username],
            )
            .await;

        if check.unwrap().len() != 0 {
            return Ok(tide::Response::new(tide::StatusCode::Unauthorized));
        }

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
