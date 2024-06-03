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


    pub async fn pptx_viewer(req: Request<()>) -> tide::Result {
        let params: PptxParams = req.query()?;
        let path = get_filepath(params.filename, params.course_id)?.replace(" ", "%20");
        print!("{}", &path[6..path.len()]);
        let path = encode(path[6..path.len()].as_ref());

        let host = "pup-settled-dolphin.ngrok-free.app".to_string();
        let office_viewer_url = format!(
            "https://view.officeapps.live.com/op/view.aspx?src={}",
            (&format!("{}/files/{}", host, path.replace("/public/", "/files/")))
        );
        print!("{}", office_viewer_url);
        Ok(Redirect::new(office_viewer_url).into())
    }
}
