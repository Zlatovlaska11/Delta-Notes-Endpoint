pub mod server {

    use serde::{Deserialize, Serialize};
    
    use tide::{http::headers::HeaderValue, security::CorsMiddleware, Response};

    use crate::{
        auth,
        filehalndler::{file_serving::file_serve::pptx_viewer, handler::course_list},
    };

    #[derive(Debug, Deserialize, Serialize)]
    struct PostParams {
        #[serde(with = "string_or_u32")]
        id: u32,
        // Add more fields as needed
    }

    // Custom serde helper to handle parsing the id field as a string or u32
    mod string_or_u32 {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            value.to_string().serialize(serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse::<u32>().map_err(serde::de::Error::custom)
        }
    }

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

        let json_list = move |mut req: tide::Request<()>| async move {
            let params: PostParams = req.body_json().await?;
            // Access the parameters

            let json = course_list(params.id as u8);

            let mut res = Response::new(200);
            res.set_body(json);

            // Set the content type to application/json
            res.set_content_type("application/json");

            Ok(res)
        };

        app.at("/auth/login").post(login);
        app.at("/auth/register").post(register);

        app.at("/pptx").get(pptx_viewer);

        app.at("/list").post(json_list);

        Ok(app.into())
    }
}
