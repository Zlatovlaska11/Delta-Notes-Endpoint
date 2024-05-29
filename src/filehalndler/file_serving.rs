pub mod file_serve {

    use tide::{self, http::url, Redirect, Request, Response, StatusCode};
    use urlencoding::encode;

    use crate::filehalndler::get_courses::courses::get_filepath;

    pub async fn serve_file(req: Request<()>) -> tide::Result {
        let filename: String = req.param("filename")?.to_string();
        let id = req.param("id")?.parse::<u8>()?;
        let file_path = get_filepath(filename, id)?;

        let file_content = async_std::fs::read(file_path).await?;

        let mut response = Response::new(StatusCode::Ok);
        response.set_body(file_content);
        response.insert_header(
            "Content-Type",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        );
        Ok(response)
    }

    fn pptx_viewer(req: Request<()>) -> tide::Result {
        let filename = req.param("filename")?;
        let id = req.param("id")?.parse::<u8>()?;
        let path = get_filepath(filename.to_string(), id);

        let host = req
            .header("host")
            .map(|h| h.as_str())
            .unwrap_or("localhost:8080");
        let office_viewer_url = format!(
            "https://view.officeapps.live.com/op/view.aspx?src={}",
            encode(&format!("{}/file/{}", host, path?))
        );
        Ok(Redirect::new(office_viewer_url).into())
    }
}
