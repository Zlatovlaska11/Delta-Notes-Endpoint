pub mod auth {

    
    use dotenv::dotenv;
    use serde_json::{json, Value};
    use std::env;

    use surrealdb::{engine::local::Db, Surreal};

    use rocket::{form::validate::Len, http::Status, serde::json::Json};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Clone)]
    pub struct Creds {
        pub username: String,
        pub password: String,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
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

    pub async fn get_connection() -> surrealdb::Result<Surreal<Db>> {
        // Create a connection string

        // Create database connection
        let db = Surreal::new::<surrealdb::engine::local::RocksDb>("./local.db").await?; // Signin as a namespace, database, or root user
        db.use_ns("delta-notes-db").use_db("users").await?;
        Ok(db)
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

    pub async fn login(creds: Creds) -> Result<Json<Value>, Status> {
        let client = get_connection().await.unwrap();

        let sql = format!(
            "SELECT * FROM users WHERE username = '{}' and password = '{}';",
            creds.username, creds.password
        );
        println!("{}", sql.clone());
        let rows = client.query(sql).await;
        dbg!(rows.as_ref().unwrap());
        let token = get_token(creds.username, creds.password).await;

        match rows {
            Ok(mut rows) => {
                let rows: Vec<Value> = rows.take(0).unwrap();
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
        let client = get_connection().await.unwrap();

        let creds_clone = creds.clone();
        let sql = format!(
            "SELECT * FROM users WHERE username = '{}' and password = '{}';",
            creds_clone.username, creds_clone.password
        );
        let check = client.query(sql).await;
        println!("{:?}", check);
        let check: Vec<CredsReg> = check.unwrap().take(0).unwrap();
        if check.len() > 0 {
            return Status::Unauthorized;
        }

        let rows = client
            .query("INSERT INTO users (username, password, mail) VALUES ($1, $2, $3);")
            .bind(&[
                &creds.username.replace("\"", "'"),
                &creds.password.replace("\"", "'"),
                &creds.mail.replace("\"", "'"),
            ])
            .await;

        match rows {
            Ok(_) => Status::Ok,
            Err(_) => Status::Unauthorized,
        }
    }
}
