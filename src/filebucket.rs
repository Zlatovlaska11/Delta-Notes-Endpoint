pub mod filebucket {

    use std::fs;
    use unofficial_appwrite::client::ClientBuilder;
    use unofficial_appwrite::error::Error;
    use unofficial_appwrite::id::ID;
    use unofficial_appwrite::services::server::storage::{self, Storage};

    pub async fn get_files() -> Result<tide::StatusCode, tide::Error> {
        let client = ClientBuilder::default()
            .set_endpoint("https://cloud.appwrite.io/v1")
            .unwrap()
            .set_project("663a5dc100346fbe368e")?
            .set_key("cd0aef75eff86d2386795a68608e05e1bd3a12e6c3081c709758373de6c27d865fa4b49954e59895339412fa87e4bc74628a4b7c2069370d60121d547e80778bfd5114b604abee1ae761d133f9b4d07511aa1d7587037e6185b46be178c96850b13e3a3503aac6b7c29cf44241107531b5802a28242ef10d8169da6cff10f4b2")?
            .build()?;

        let bucket_id = "663a5dc100346fbe368e";

        let upload = Storage::create_files(&client, bucket_id, ID::unique(), "/home/zltatovlas/Downloads/IMG_8522.jpeg", None, None,).await?;

        dbg!(upload);

        let get_file = Storage::get_file(&client, "661...e9", "661...71e").await?;
        dbg!(get_file);
        Ok(tide::StatusCode::Ok)
        //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
    }
}
