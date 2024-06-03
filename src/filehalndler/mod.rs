pub mod file_list;
pub mod file_serving;
pub mod get_courses;
mod test;

pub mod handler {
    use std::u8;

    use serde_json::json;

    use super::file_list::get_files::get_list;

    pub fn course_list(id: u8) {
        if id == u8::MAX {
            todo!("return err");
        }

        let json_list = json!(get_list(id, None).unwrap());

        println!("{}", json_list);
    }
}
