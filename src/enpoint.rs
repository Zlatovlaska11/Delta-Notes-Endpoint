pub mod server {

    use tide::{http::headers::HeaderValue, security::CorsMiddleware};

    use crate::auth::{self};

    pub async fn start_server(conn_str: String) -> shuttle_tide::ShuttleTide<()> {
        let mut app = tide::new();

        println!("server bound");

        app.with(
            CorsMiddleware::new()
                .allow_methods(
                    "GET, POST, PUT, DELETE, OPTIONS"
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_origin("*")
                .allow_credentials(true)
                .allow_headers(
                    "Content-Type, Authorization"
                        .parse::<HeaderValue>()
                        .unwrap(),
                ),
        );

        let login = move |req: tide::Request<()>| {
            let conn_str = conn_str.clone(); // Clone the connection string to move into the closure

            async move {
                println!("request accepted");
                auth::auth::login(req, conn_str).await // Call the login function from within the closure
            }
        };
        app.at("/auth/login").post(login);

        Ok(app.into())
    }
}
