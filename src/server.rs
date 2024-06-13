pub mod server_rocket {

    use core::fmt;
    use dotenv::dotenv;
    use rocket::http::uri::Query;
    use serde_json::Value;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::auth::auth::{login, TokenClaims};
    use crate::auth::auth::{register, CredsReg};
    use crate::filehalndler::file_serving::file_serve::pptx_viewer;
    use crate::filehalndler::handler::course_list;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use rocket::http::Status;
    use rocket::serde::json::Json;
    use rocket::{options, post, FromForm};
    use serde::{Deserialize, Serialize};

    use crate::auth::auth::Creds;

    #[derive(Debug, Deserialize, Serialize, FromForm)]
    pub struct PostParams {
        #[serde(with = "string_or_u32")]
        pub id: u32,
        // Add more fields as needed
    }

    #[derive(Debug, Deserialize, Serialize, FromForm)]
    pub struct Token {
        token: String,
    }

    #[derive(Debug, Deserialize, FromForm, Clone)]
    pub struct PptxParams {
        pub filename: String,
        pub id: u8,
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

    #[post("/auth/login", data = "<creds>")]
    pub async fn login_endpoint(creds: Json<Creds>) -> Result<Json<serde_json::Value>, Status> {
        let creds_log: Creds = Creds {
            username: creds.username.to_string(),
            password: creds.password.to_string(),
        };

        let status = login(creds_log).await;

        match status {
            Ok(token) => Ok(token),
            Err(stat) => Err(stat),
        }
    }

    #[post("/auth/reg", data = "<creds>")]
    pub async fn reg_endpoint(creds: Json<CredsReg>) -> Status {
        let creds_log: CredsReg = CredsReg {
            username: creds.username.to_string(),
            password: creds.password.to_string(),
            mail: creds.mail.to_string(),
        };

        let status = register(creds_log).await;

        status
    }

    #[post("/list", data = "<id>")]
    pub fn list<'a>(id: Json<PostParams>) -> Json<serde_json::Value> {
        let id = id.id as u8;

        let list = course_list(id);

        Json(list)
    }

    use rocket::get;
    #[get("/pptx?<id>&<filename>")]
    pub async fn get_pptx_link(id: u8, filename: String) -> Result<Json<String>, Status> {
        let params = PptxParams { filename, id };
        let Ok(list) = pptx_viewer(params).await else {
            return Err(Status::Unauthorized);
        };

        return Ok(list);
    }

    #[post("/check", data = "<token>")]
    pub async fn validate_token(token: Json<Token>) -> (Status, Json<Value>) {
        dotenv().ok();
        let key = env::var("SECRET").unwrap();

        println!("{}", token.token);
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = decode::<TokenClaims>(
            &token.token,
            &DecodingKey::from_secret(key.as_ref()),
            &validation,
        );

        match token_data {
            Ok(c) => {
                // Check if the token has expired
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize;
                if c.claims.exp < current_time {
                    (Status::ImATeapot, Json(serde_json::json!("expired")))
                } else {
                    (Status::Ok, Json(serde_json::json!("ok")))
                }
            }
            Err(err) => (Status::ImATeapot, Json(serde_json::json!(err.to_string()))),
        }
    }

    #[options("/<_..>")]
    pub fn all_options() {
        /* Intentionally left empty */
    }
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
