pub mod server {

    use std::{fs::File, io::Read};

    use tide::{
        http::headers::HeaderValue, security::CorsMiddleware, Request, Response, StatusCode,
    };

    use crate::auth;

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

        app.at("/auth/login").post(login);
        app.at("/auth/register").post(register);
        



        app.at("/file/:filename").get(|req: Request<()>| async move {

        let filename: String = req.param("filename").unwrap_or_default().to_string();

        let filename = format!("public/files/{}", filename);
        if let Ok(mut file) = File::open(&filename) {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            let content_type = match filename.split('.').last() {
                Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                _ => "application/octet-stream", // fallback to binary data
            };

            let mut response = Response::new(StatusCode::Ok);
            response.set_body(contents);
            response.insert_header("Content-Type", content_type);

            Ok(response)
        } else {
            Ok(Response::new(StatusCode::NotFound))
        }
    });
        Ok(app.into())
    }
}
