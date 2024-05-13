pub mod server {

    use tide::{http::headers::HeaderValue, security::CorsMiddleware};

    use crate::{auth, filebucket};

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

        let cns = conn_str.clone(); // Clone the connection string to move into the closure
        let login = move |req: tide::Request<()>| {
            let constr = conn_str.clone(); // Clone the connection string to move into the closure

            async move {
                println!("request accepted");
                auth::auth::login(req, constr).await // Call the login function from within the closure
            }
        };

        let register = move |req: tide::Request<()>| {
            let cnstr = cns.clone();
            async move { auth::auth::register(req, cnstr).await }
        };

        let upload = move |reg: tide::Request<()>| {
            let status = filebucket::filebucket::get_files();
            async move {
                
                status.await
            }
        };

        app.at("/auth/login").post(login);
        app.at("/auth/register").post(register);
        app.at("/file/upload").post(upload);

        Ok(app.into())
    }
}
