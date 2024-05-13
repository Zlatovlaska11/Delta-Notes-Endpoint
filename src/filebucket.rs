pub mod filebucket {

    use std::fs;
    use unofficial_appwrite::client::ClientBuilder;
    use unofficial_appwrite::error::Error;
    use unofficial_appwrite::id::ID;
    use unofficial_appwrite::services::server::storage::{self, Storage};

    pub async fn get_files() -> Result<tide::StatusCode, tide::Error> {
        let client = ClientBuilder::default()
            .set_endpoint("https://cloud.appwrite.io/v1")
            .expect("why?")
            .set_project("663a5d5b002847fb669c")?
            .build()?;

        let bucket_id = "663a5d5b002847fb669c";

        let upload = Storage::create_files(&client, bucket_id, ID::unique(), "/home/zltatovlas/Downloads/IMG_8522.jpeg", None, Some(|prog| {
            println!("{}:{}:{}", prog.id, prog.progress, prog.size_uploaded);
        })).await?;
        dbg!(upload.name);


        //let get_file = Storage::get_file(&client, "661...e9", "661...71e").await?;
        //dbg!(get_file);
        Ok(tide::StatusCode::Ok)
        //.set_self_signed(false)? // Use only on dev mode with a self-signed SSL cert
    }
}
