pub mod file_list;
pub mod file_serving;
pub mod get_courses;
mod test;

use crate::filehalndler::get_courses::courses;

pub mod handler {
    use super::get_courses;

    pub fn get_file(req: tide::Request<()>) -> tide::Response {
        let id = req
            .param("id")
            .expect("id is not included")
            .to_string()
            .parse::<u8>()
            .unwrap_or_else(|_x| u8::MAX);

        if id == u8::MAX {
            return tide::Response::new(tide::StatusCode::BadRequest);
        }

        let filename = req
            .param("filename")
            .expect("id is not included")
            .to_string();

        let filepath = get_courses::courses::get_filepath(filename, id).unwrap();

        tide::Response::new(200)
        
    }
}
