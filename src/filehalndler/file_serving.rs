pub mod file_serve {

    use serde::Deserialize;
    use tide::{self, Request, Response};
    use urlencoding::encode;

    use crate::filehalndler::get_courses::courses::get_filepath;

    #[derive(Debug, Deserialize)]
    struct PptxParams {
        filename: String,
        id: u8,
    }

    pub async fn pptx_viewer(req: Request<()>) -> tide::Result {
        let params: PptxParams = req.query()?;
        let path = get_filepath(params.filename, params.id)?;
        print!("{}", &path[6..path.len()]);
        let path = encode(path[6..path.len()].as_ref());

        let host = "pup-settled-dolphin.ngrok-free.app".to_string();
        let office_viewer_url = format!("{}/files/{}", host, path.replace("/public/", "/files/"));
        print!("{}", office_viewer_url);
        let mut resp = Response::new(200);
        resp.set_body(office_viewer_url);
        Ok(resp.into())
    }
}
