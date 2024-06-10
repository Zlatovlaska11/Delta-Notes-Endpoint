pub mod auth {

    use core::panic;
    use serde_json::Value;
    use std::borrow::BorrowMut;
    use surrealdb::{
        engine::{any, remote::ws::Ws},
        opt::auth::Root,
        Surreal,
    };

    use rocket::{form::validate::Len, http::Status};
    use serde::{Deserialize, Serialize};
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
        pub mail: String,
        pub exp: usize,
    }

    pub async fn get_connection(
        conn_str: String,
    ) -> surrealdb::Result<Surreal<surrealdb::engine::remote::ws::Client>> {
        // Create a connection string

        let db = Surreal::new::<Ws>("127.0.0.1:6969").await?;
        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        Ok(db)
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
        let client = get_connection(conn_str).await.unwrap();

        let mut res = client
            .query("SELECT * FROM users WHERE username = $username AND password = $password")
            .bind((("username", lc.username), ("password", lc.password)))
            .await
            .unwrap();

        let resp: Vec<CredsReg> = res.take(0).unwrap();
        if resp.len() > 1 {
            panic!("how that this happened");
        }

        resp[0].clone()
    }

    pub async fn login(creds: Creds, conn_str: String) -> Status {
        let client = get_connection(conn_str).await.unwrap();

        let rows = client
            .query("SELECT * FROM users WHERE password = $1 and username = $2;")
            .bind(&[
                &creds.password.replace("\"", "'"),
                &creds.username.replace("\"", "'"),
            ])
            .await;

        match rows {
            Ok(mut rows) => {
                let rows: Vec<Value> = rows.take(0).unwrap();
                if rows.len() == 1 {
                    Status::Ok
                } else {
                    Status::Unauthorized
                }
            }
            Err(_) => Status::Unauthorized,
        }
    }

    pub async fn register(creds: CredsReg, conn_str: String) -> Status {
        let client = get_connection(conn_str.clone()).await;

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
