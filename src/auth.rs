pub mod auth {

    use core::panic;
    use std::env;

    use rocket::{http::Status, serde::json::Json};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use tokio_postgres::{Client, NoTls};

    #[derive(Deserialize, Serialize, Clone)]
    pub struct Creds {
        pub username: String,
        pub password: String,
    }

    #[derive(Deserialize, Serialize, Clone)]
    pub struct CredsReg {
        pub username: String,
        pub password: String,
        pub mail: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct TokenClaims {
        pub username: String,
        pub password: String,
        pub exp: usize,
    }

    use dotenv::dotenv;
    pub async fn get_connection() -> Client {
        // Create a connection string

        // Parse the connection string
        let (client, connection) = tokio_postgres::connect(
            &"postgresql://postgres:mysecretpassword@localhost:5433/postgres",
            NoTls,
        )
        .await
        .unwrap();

        // Spawn a task to process the connection
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
    }

    pub async fn get_token(username: String, password: String) -> String {
        let exp = (chrono::Utc::now().naive_utc() + chrono::naive::Days::new(1))
            .and_utc()
            .timestamp() as usize;

        let claims = TokenClaims {
            username,
            password,
            exp,
        };
        dotenv().ok(); // This line loads the environment variables from the ".env" file.
        let secret = env::var("SECRET").unwrap();
        let jwt = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        jwt
    }

    async fn get_info(lc: Creds) -> CredsReg {
        let client = get_connection().await;

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

    pub async fn login(creds: Creds) -> Result<Json<Value>, Status> {
        let client = get_connection().await;

        let token = get_token(creds.username.clone(), creds.password.clone()).await;

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
                if rows.len() == 1 {
                    Ok(Json(json!(token)))
                } else {
                    Err(Status::Unauthorized)
                }
            }
            Err(_) => Err(Status::Unauthorized),
        }
    }

    pub async fn register(creds: CredsReg) -> Status {
        let client = get_connection().await;

        let creds_clone = creds.clone();
        let check = client
            .query(
                "SELECT * FROM users WHERE username = $1",
                &[&creds_clone.username],
            )
            .await;

        if check.unwrap().len() != 0 {
            return Status::Unauthorized;
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
            Ok(_) => Status::Ok,
            Err(_) => Status::Unauthorized,
        }
    }
}
