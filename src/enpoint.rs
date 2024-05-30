pub mod server {

    use std::{fs::File, io::Read};

    use tide::{
        http::headers::HeaderValue, security::CorsMiddleware, Request, Response, StatusCode,
    };

    use crate::{
        auth, filehalndler::file_serving::file_serve::pptx_viewer,
        filehalndler::file_serving::file_serve::serve_file,
    };

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

        app.at("file/:filename").get(serve_file);
        app.at("/pptx").get(pptx_viewer);

        app.at("/fls/:filename").get(|req: Request<()>| async move {
            // Extract filename from the request path
            let filename: String = req.param("filename").unwrap_or_default().to_string();
            let filename = urlencoding::decode(&filename).unwrap();

            // Attempt to open the file
            if let Ok(mut file) = File::open(&*filename) {
                let mut contents = Vec::new();
                // Read the file contents into a buffer
                file.read_to_end(&mut contents)?;

                // Set content type based on file extension
                let content_type = match filename.split('.').last() {
                    Some("pptx") => {
                        "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                    }
                    _ => "application/octet-stream", // fallback to binary data
                };

                // Return the file contents in the response body
                let mut response = Response::new(StatusCode::Ok);
                response.set_body(contents);
                response.insert_header("Content-Type", content_type);

                Ok(response)
            } else {
                // Return a 404 Not Found if the file does not exist
                Ok(Response::new(StatusCode::NotFound))
            }
        });

        Ok(app.into())
    }
}
