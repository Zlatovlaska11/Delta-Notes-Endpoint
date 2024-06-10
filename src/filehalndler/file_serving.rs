pub mod file_serve {

    use rocket::http::Status;
    use rocket::serde::json::Json;
    use urlencoding::encode;

    use crate::filehalndler::get_courses::courses::get_filepath;
    use crate::server::server_rocket::PptxParams;

    pub async fn pptx_viewer(params: PptxParams) -> Result<Json<String>, rocket::http::Status> {
        let Ok(path) = get_filepath(params.filename, params.id) else {
            return Err(Status::BadRequest);
        };

        print!("{}", &path[6..path.len()]);
        let path = encode(path[6..path.len()].as_ref());

        let host = "pup-settled-dolphin.ngrok-free.app".to_string();
        let office_viewer_url = format!("{}/files/{}", host, path.replace("/public/", "/files/"));
        print!("{}", office_viewer_url);
        Ok(Json(office_viewer_url))
    }
}
