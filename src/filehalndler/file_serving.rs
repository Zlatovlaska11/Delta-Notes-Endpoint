pub mod file_serve {

    use serde::Deserialize;
    use tide::{self, Redirect, Request, Response, StatusCode};
    use urlencoding::encode;

    use crate::filehalndler::get_courses::courses::get_filepath;

    #[derive(Debug, Deserialize)]
    struct PptxParams {
        filename: String,
        course_id: u8,
    }

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

    pub async fn pptx_viewer(req: Request<()>) -> tide::Result {
        let params: PptxParams = req.query()?;
        let path = get_filepath(params.filename, params.course_id)?.replace(" ", "%20");
        print!("{}", path);
        let path = encode(path.as_ref());

        //NEEDS TO BE DELETED
        let host = "https://zlatovlas-delta-notes.shuttleapp.rs".to_string();
        let office_viewer_url = format!(
            "https://view.officeapps.live.com/op/view.aspx?src={}",
            (&format!("{}/fls/{}", host, path))
        );
        print!("{}", office_viewer_url);
        Ok(Redirect::new(office_viewer_url).into())
    }
}
