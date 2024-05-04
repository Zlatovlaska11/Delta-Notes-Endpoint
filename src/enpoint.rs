pub mod server {

    use crate::auth::{self};

    pub async fn start_server(conn_str: String) -> shuttle_tide::ShuttleTide<()> {
        let mut app = tide::new();

        println!("server bound");

        app.with(tide::log::LogMiddleware::new());

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
