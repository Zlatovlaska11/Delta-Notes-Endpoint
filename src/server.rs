pub mod server_rocket {

    use crate::auth::auth::login;
    use crate::auth::auth::{register, CredsReg};
    use crate::filehalndler::file_serving::file_serve::pptx_viewer;
    use crate::filehalndler::handler::course_list;
    use rocket::http::Status;
    use rocket::serde::json::Json;
    use rocket::{post, FromForm};
    use serde::{Deserialize, Serialize};

    use crate::auth::auth::Creds;

    #[derive(Debug, Deserialize, Serialize, FromForm)]
    pub struct PostParams {
        #[serde(with = "string_or_u32")]
        pub id: u32,
        // Add more fields as needed
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
    pub async fn login_endpoint(creds: Json<Creds>) -> Status {
        let conn_str = std::env::var("POSTGRES_URL").expect("no postgres url specified");

        let creds_log: Creds = Creds {
            username: creds.username.to_string(),
            password: creds.password.to_string(),
        };

        let status = login(creds_log, conn_str.to_string()).await;

        status
    }

    #[post("/auth/reg", data = "<creds>")]
    pub async fn reg_endpoint(creds: Json<CredsReg>) -> Status {
        let conn_str = std::env::var("POSTGRES_URL").expect("no postgres url specified");

        let creds_log: CredsReg = CredsReg {
            username: creds.username.to_string(),
            password: creds.password.to_string(),
            mail: creds.mail.to_string(),
        };

        let status = register(creds_log, conn_str.to_string()).await;

        status
    }

    #[post("/list", data = "<id>")]
    pub fn list<'a>(id: Json<PostParams>) -> Json<String> {
        let id = id.id as u8;

        let list = course_list(id);

        Json(list.to_string())
    }

    #[post("/pptx", data = "<data>")]
    pub async fn get_pptx_link(data: Json<PptxParams>) -> Result<Json<String>, Status> {
        let data = data.clone();
        let params = PptxParams {
            filename: data.filename.clone(),
            id: data.id,
        };
        let Ok(list) = pptx_viewer(params).await else {
            return Err(Status::Unauthorized);
        };

        return Ok(list);
    }
}
