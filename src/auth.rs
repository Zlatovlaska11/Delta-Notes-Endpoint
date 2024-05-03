pub mod auth {
    use serde::Deserialize;
    use tide::{http::Response, ResponseBuilder};
    use tokio_postgres::{row, Client, GenericClient, NoTls};
    use crate::database::db_work::get_connection;

    #[derive(Deserialize)]
    struct Creds {
        username: String,
        password: String,
    }

    pub async fn login(mut req: tide::Request<()>, conn_str: String) -> tide::Result<tide::Response> {
        let creds: Creds = req.body_json().await.unwrap();
        let client = get_connection(conn_str).await;

        let rows = client
            .query(
                "SELECT * FROM users WHERE password = $1 and username = $2;",
                &[&creds.password, &creds.username],
            )
            .await;

        match rows {
            Ok(_) => {
                let mut response = tide::Response::new(tide::StatusCode::Ok);

                response.set_body("hello");

                Ok(response)
            }
            Err(_) => Ok(tide::Response::new(tide::StatusCode::Unauthorized))
        }
    }
}
