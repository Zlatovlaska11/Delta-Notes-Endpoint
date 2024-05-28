pub mod file_list;
pub mod get_courses;
mod test;

use crate::filehalndler::get_courses::courses;

pub mod handler {

    pub fn get_file_list(req: tide::Request<()>) -> tide::Response {
        let id = req
            .param("id")
            .expect("id is not included")
            .to_string()
            .parse::<u8>()
            .unwrap_or_else(|_x| u8::MAX);

        if id == u8::MAX {
            return tide::Response::new(tide::StatusCode::BadRequest)
        }

        tide::Response::new(200)
    }
}
